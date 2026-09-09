//! 🧮️ Trinity jack query executor.
#![allow(dead_code)]

use crate::apply_trinity_graph_mutations;
use crate::standards::v1::subsets::any::schema::mutations::{change_data_property, create_edge, create_node, delete_node, move_node, rename_node, TrinityGraphMutation};
use crate::{port_key, Edge, EntityRef, Graph, JackSnapshot, Node, Port, PortDirection, PropertyBag, PropertyValue};
use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{Clause, Expr, Pattern, Query, QueryResult, ReturnItem};
use crate::language_service::parse;

/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

#[path = "🪜️execution/🦀️.rs"]
mod execution;
pub use execution::{QueryExecution, QueryExecutionPreparation, QueryPreparationStep};
pub(crate) use execution::QUERY_OUTPUT_MAXIMUM_BYTES;

/// ▶️ Execute a jack query against a graph and emit CQRS operations for mutations.
pub fn execute(graph: &Graph, query: &Query) -> Result<(QueryResult, Vec<TrinityGraphMutation>), String> {
    let mut fixture = graph.to_fixture();
    let mut view = graph.clone();
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    let mut operations = Vec::new();
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
                let batch = emit_create_operations(&fixture, pattern)?;
                operations.extend(batch.iter().cloned());
                fixture = apply_trinity_graph_mutations(fixture, &batch).map_err(|e| e.to_string())?;
                view = Graph::from_fixture(fixture.clone()).map_err(|e| e.to_string())?;
            }
            Clause::Delete(vars) => {
                for var in vars {
                    if let Some(id) = bindings.first().and_then(|b| b.nodes.get(var).cloned()) {
                        let operation = delete_node(id);
                        operations.push(operation.clone());
                        fixture = apply_trinity_graph_mutations(fixture, std::slice::from_ref(&operation)).map_err(|e| e.to_string())?;
                        view = Graph::from_fixture(fixture.clone()).map_err(|e| e.to_string())?;
                    }
                }
            }
            Clause::Set(items) => {
                let b = bindings.first().cloned().unwrap_or_default();
                for item in items {
                    if let Some(node_id) = b.nodes.get(&item.var) {
                        let operation = emit_set_operation(&fixture, node_id, &item.prop, item.value.clone())?;
                        operations.push(operation.clone());
                        fixture = apply_trinity_graph_mutations(fixture, std::slice::from_ref(&operation)).map_err(|e| e.to_string())?;
                        view = Graph::from_fixture(fixture.clone()).map_err(|e| e.to_string())?;
                    }
                }
            }
            Clause::Merge(pattern) => {
                let existing = match_patterns(&view, std::slice::from_ref(pattern))?;
                if existing.is_empty() {
                    let batch = emit_create_operations(&fixture, pattern)?;
                    operations.extend(batch.iter().cloned());
                    fixture = apply_trinity_graph_mutations(fixture, &batch).map_err(|e| e.to_string())?;
                    view = Graph::from_fixture(fixture.clone()).map_err(|e| e.to_string())?;
                }
            }
        }
    }
    if let Some(items) = return_items {
        return Ok((build_return(&view, &bindings, &items), operations));
    }
    Ok((QueryResult::table(vec![], vec![]), operations))
}

/// ▶️ Parse and execute jack in one step.
pub fn run(graph: &mut Graph, source: &str) -> Result<QueryResult, String> {
    let query = parse(source)?;
    let (result, operations) = execute(graph, &query)?;
    if !operations.is_empty() {
        let fixture = apply_trinity_graph_mutations(graph.to_fixture(), &operations).map_err(|e| e.to_string())?;
        *graph = Graph::from_fixture(fixture).map_err(|e| e.to_string())?;
    }
    Ok(result)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_json(graph: &mut Graph, source: &str) -> Result<String, String> {
    let result = run(graph, source)?;
    Ok(pack::to_json_string(&result))
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

fn emit_set_operation(fixture: &JackSnapshot, node_id: &str, prop: &str, value: PropertyValue) -> Result<TrinityGraphMutation, String> {
    let scene = crate::jack_working_scene(fixture);
    let node = scene.nodes.iter().find(|node| node.id == node_id).ok_or_else(|| format!("node {node_id} not found"))?;
    match prop {
        "name" => {
            let PropertyValue::String(name) = value else {
                return Err(format!("node {node_id}.name expects string value"));
            };
            Ok(rename_node(node_id.to_string(), name))
        }
        "x" => {
            let x = value.as_f64().ok_or_else(|| format!("node {node_id}.x expects number value"))?;
            Ok(move_node(node_id.to_string(), x, node.y))
        }
        "y" => {
            let y = value.as_f64().ok_or_else(|| format!("node {node_id}.y expects number value"))?;
            Ok(move_node(node_id.to_string(), node.x, y))
        }
        _ => Ok(change_data_property(EntityRef::Node(node_id.to_string()), prop.to_string(), value)),
    }
}

fn emit_create_operations(fixture: &JackSnapshot, pattern: &Pattern) -> Result<Vec<TrinityGraphMutation>, String> {
    let scene = crate::jack_working_scene(fixture);
    let left = pattern.nodes.first().ok_or_else(|| "empty create pattern".to_string())?;
    let left_id = format!("{}-{}", left.var, scene.nodes.len());
    let mut operations = Vec::new();
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    }
    operations.push(create_node(Node { id: left_id.clone(), kind: left.kind.clone(), name: left.var.clone(), x: scene.nodes.len() as f64 * 120.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: left_ports }));
    if let Some(edge_pat) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pat.right.var, scene.nodes.len() + 1);
        operations.push(create_node(Node {
            id: right_id.clone(),
            kind: edge_pat.right.kind.clone(),
            name: edge_pat.right.var.clone(),
            x: (scene.nodes.len() + 1) as f64 * 120.0,
            y: 80.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
        }));
        operations.push(create_edge(Edge {
            id: format!("e-{}", scene.edges.len()),
            kind: edge_pat.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        }));
    }
    Ok(operations)
}
// #endregion 🔖️Executor
// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
