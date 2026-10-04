//! 🧮️ Trinity jack query executor.
#![allow(dead_code)]

use crate::{Edge, EntityRef, Graph, Node, PropertyValue};
use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{Clause, Expr, Pattern, Query, QueryResult, ReturnItem};
use crate::language_service::parse;

/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

/// 🧮️ One effect a query applies to its working graph. A run publishes its effects as graph leaves of the document's
/// composed `content` child (`crate::graph_leaves`), never as parent-lane leaves (design §20.15).
#[derive(Clone, Debug, PartialEq)]
pub enum GraphEffect {
    CreateNode(Node),
    DeleteNode(String),
    CreateEdge(Edge),
    DeleteEdge(String),
    RenameNode { id: String, name: String },
    MoveNode { id: String, x: f64, y: f64 },
    SetProperty { entity: EntityRef, key: String, value: PropertyValue },
    RemoveProperty { entity: EntityRef, key: String },
}

#[path = "🪜️execution/🦀️.rs"]
mod execution;
pub use execution::{QueryExecution, QueryExecutionPreparation, QueryPreparationStep};
pub(crate) use execution::QUERY_OUTPUT_MAXIMUM_BYTES;
use execution::{emit_create_operations_from_graph, emit_set_operation_from_graph};

/// ▶️ Executes a jack query against a graph and returns the effects its mutating clauses apply.
pub fn execute(graph: &Graph, query: &Query) -> Result<(QueryResult, Vec<GraphEffect>), String> {
    let mut view = graph.clone();
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    let mut effects = Vec::new();
    let apply = |view: &mut Graph, batch: Vec<GraphEffect>, effects: &mut Vec<GraphEffect>| -> Result<(), String> {
        crate::apply_graph_effects(view, &batch).map_err(|e| e.to_string())?;
        effects.extend(batch);
        Ok(())
    };
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => {
                bindings = match_patterns(&view, patterns)?;
            }
            Clause::Where(expr) => {
                bindings.retain(|b| eval_expr(&view, b, expr));
            }
            Clause::Return(items) => {
                return_items = Some(items.clone());
            }
            Clause::Create(pattern) => {
                let batch = emit_create_operations_from_graph(&view, pattern).map_err(|e| e.into_message())?;
                apply(&mut view, batch, &mut effects)?;
            }
            Clause::Delete(vars) => {
                for var in vars {
                    if let Some(id) = bindings.first().and_then(|b| b.nodes.get(var).cloned()) {
                        apply(&mut view, vec![GraphEffect::DeleteNode(id)], &mut effects)?;
                    }
                }
            }
            Clause::Set(items) => {
                let b = bindings.first().cloned().unwrap_or_default();
                for item in items {
                    if let Some(node_id) = b.nodes.get(&item.var) {
                        let effect = emit_set_operation_from_graph(&view, node_id, &item.prop, item.value.clone()).map_err(|e| e.into_message())?;
                        apply(&mut view, vec![effect], &mut effects)?;
                    }
                }
            }
            Clause::Merge(pattern) => {
                if match_patterns(&view, std::slice::from_ref(pattern))?.is_empty() {
                    let batch = emit_create_operations_from_graph(&view, pattern).map_err(|e| e.into_message())?;
                    apply(&mut view, batch, &mut effects)?;
                }
            }
        }
    }
    if let Some(items) = return_items {
        return Ok((build_return(&view, &bindings, &items), effects));
    }
    Ok((QueryResult::table(vec![], vec![]), effects))
}

/// ▶️ Parses and executes jack in one step, applying its effects to `graph`.
pub fn run(graph: &mut Graph, source: &str) -> Result<QueryResult, String> {
    let query = parse(source)?;
    let (result, effects) = execute(graph, &query)?;
    crate::apply_graph_effects(graph, &effects).map_err(|e| e.to_string())?;
    Ok(result)
}

/// ▶️ Executes jack and returns the JSON result.
pub fn run_json(graph: &mut Graph, source: &str) -> Result<String, String> {
    let result = run(graph, source)?;
    Ok(semio_framework_pack_json::to_json_string(&result))
}

fn match_patterns(graph: &Graph, patterns: &[Pattern]) -> Result<Vec<Binding>, String> {
    let mut bindings = vec![Binding::default()];
    for pattern in patterns {
        let mut next = Vec::new();
        for binding in &bindings {
            next.extend(match_pattern(graph, pattern, binding)?);
        }
        bindings = next;
    }
    Ok(bindings)
}

fn match_pattern(graph: &Graph, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, String> {
    let left = pattern.nodes.first().ok_or_else(|| "empty pattern".to_string())?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for (node_id, node) in &graph.nodes {
            if node.kind != left.kind {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id) {
                continue;
            }
            for (edge_id, edge) in &graph.edges {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let src = crate::port_node_id(&edge.source);
                let tgt = crate::port_node_id(&edge.target);
                if src != Some(node_id.as_str()) {
                    continue;
                }
                let Some(tgt_id) = tgt else { continue };
                let Some(tgt_node) = graph.nodes.get(tgt_id) else { continue };
                if tgt_node.kind != edge_pat.right.kind {
                    continue;
                }
                let mut b = base.clone();
                b.nodes.insert(left.var.clone(), node_id.clone());
                if let Some(ev) = &edge_pat.var {
                    b.edges.insert(ev.clone(), edge_id.clone());
                }
                if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                    continue;
                }
                b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                out.push(b);
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for (node_id, node) in &graph.nodes {
        if node.kind != left.kind {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id.clone());
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr(graph: &Graph, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value(graph: &Graph, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    let node_id = binding.nodes.get(var)?;
    let node = graph.node(node_id)?;
    match prop {
        "id" => Some(PropertyValue::String(node.id.clone())),
        "name" => Some(PropertyValue::String(node.name.clone())),
        "kind" => Some(PropertyValue::String(node.kind.clone())),
        _ => node.properties.get(prop).cloned(),
    }
}

fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    items.iter().any(|item| {
        let ReturnItem::Var(v) = item else { return false };
        bindings.iter().any(|b| binding_has_entity(b, v))
    })
}

fn collect_graph_entities(bindings: &[Binding], items: &[ReturnItem]) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut node_ids = BTreeSet::new();
    let mut edge_ids = BTreeSet::new();
    for binding in bindings {
        for item in items {
            if let ReturnItem::Var(v) = item {
                if let Some(id) = binding.nodes.get(v) {
                    node_ids.insert(id.clone());
                }
                if let Some(id) = binding.edges.get(v) {
                    edge_ids.insert(id.clone());
                }
            }
        }
    }
    (node_ids, edge_ids)
}

fn build_return(graph: &Graph, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        let graph_fixture = graph.subgraph_fixture(&node_ids, &edge_ids);
        return QueryResult::graph(columns, graph_fixture);
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            let val = match item {
                ReturnItem::Var(v) => binding.nodes.get(v).and_then(|id| graph.node(id)).map_or(PropertyValue::Null, |n| PropertyValue::String(n.name.clone())),
                ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop).unwrap_or(PropertyValue::Null),
            };
            row.push(val);
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}

// #endregion 🔖️Executor
// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
