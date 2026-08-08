//! 🧮️ Trinity jack query executor.
#![allow(dead_code)]

use crate::artifacts::jack::mutations::{apply_trinity_graph_mutations, TrinityGraphMutation};
use crate::artifacts::jack::{port_key, Camera, Edge, EntityRef, Graph, JackSnapshot, Manifest, Node, Port, PortDirection, PropertyBag, PropertyValue};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::ast::{Assignment, Clause, Expr, Pattern, PatternEdge, PatternNode, Query, QueryResult, QueryResultKind, ReturnItem};
use crate::language_service::parse;
use crate::lexer::{lex_spanned, Token, SpannedToken};

/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

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
                        let operation = TrinityGraphMutation::DeleteNode { id };
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
    serde_json::to_string(&result).map_err(|e| e.to_string())
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
                let src = crate::artifacts::jack::port_node_id(&edge.source);
                let tgt = crate::artifacts::jack::port_node_id(&edge.target);
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
    let node = fixture.nodes.iter().find(|node| node.id == node_id).ok_or_else(|| format!("node {node_id} not found"))?;
    match prop {
        "name" => {
            let PropertyValue::String(name) = value else {
                return Err(format!("node {node_id}.name expects string value"));
            };
            Ok(TrinityGraphMutation::Rename { id: node_id.to_string(), name })
        }
        "x" => {
            let x = value.as_f64().ok_or_else(|| format!("node {node_id}.x expects number value"))?;
            Ok(TrinityGraphMutation::Reposition { id: node_id.to_string(), x, y: node.y })
        }
        "y" => {
            let y = value.as_f64().ok_or_else(|| format!("node {node_id}.y expects number value"))?;
            Ok(TrinityGraphMutation::Reposition { id: node_id.to_string(), x: node.x, y })
        }
        _ => Ok(TrinityGraphMutation::SetDataProperty { entity: EntityRef::Node(node_id.to_string()), key: prop.to_string(), value }),
    }
}

fn emit_create_operations(fixture: &JackSnapshot, pattern: &Pattern) -> Result<Vec<TrinityGraphMutation>, String> {
    let left = pattern.nodes.first().ok_or_else(|| "empty create pattern".to_string())?;
    let left_id = format!("{}-{}", left.var, fixture.nodes.len());
    let mut operations = Vec::new();
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    }
    operations.push(TrinityGraphMutation::CreateNode { id: left_id.clone(), kind: left.kind.clone(), name: left.var.clone(), x: fixture.nodes.len() as f64 * 120.0, y: 0.0, width: 80.0, height: 40.0, ports: left_ports });
    if let Some(edge_pat) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pat.right.var, fixture.nodes.len() + 1);
        operations.push(TrinityGraphMutation::CreateNode {
            id: right_id.clone(),
            kind: edge_pat.right.kind.clone(),
            name: edge_pat.right.var.clone(),
            x: (fixture.nodes.len() + 1) as f64 * 120.0,
            y: 80.0,
            width: 80.0,
            height: 40.0,
            ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
        });
        operations.push(TrinityGraphMutation::CreateEdge {
            id: format!("e-{}", fixture.edges.len()),
            kind: edge_pat.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        });
    }
    Ok(operations)
}
// #endregion 🔖️Executor
// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::language_service::{complete, format as format_source, hover, lint, semantic_tokens};
    use crate::lexer::{lex, tokenize, TokenClass};

    fn mini_graph() -> Graph {
        let fixture = JackSnapshot {
            schema: JackSnapshot::SCHEMA.into(),
            name: "mini".into(),
            manifest_id: Some("nakagin".into()),
            manifest: Manifest::nakagin_default(),
            camera: Camera::default(),
            root_node_id: Some("root".into()),
            nodes: vec![
                Node {
                    id: "root".into(),
                    kind: "Piece".into(),
                    name: "core".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() }],
                },
                Node {
                    id: "child".into(),
                    kind: "Piece".into(),
                    name: "capsule".into(),
                    x: 120.0,
                    y: 0.0,
                    width: 80.0,
                    height: 40.0,
                    properties: PropertyBag::new(),
                    ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
                },
            ],
            edges: vec![Edge {
                id: "e1".into(),
                kind: "Connection".into(),
                source: "root@out".into(),
                target: "child@in".into(),
                properties: {
                    let mut p = PropertyBag::new();
                    p.insert("u".into(), PropertyValue::Number(1.0));
                    p.insert("v".into(), PropertyValue::Number(2.0));
                    p
                },
            }],
        };
        let mut g = Graph::from_fixture(fixture).unwrap();
        g.recompute_derived();
        g
    }

    #[test]
    fn parse_match_return() {
        let q = parse("MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
        assert_eq!(q.clauses.len(), 2);
    }

    #[test]
    fn run_match_return() {
        let mut g = mini_graph();
        let result = run(&mut g, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a.name, b.name").unwrap();
        assert_eq!(result.kind, QueryResultKind::Table);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0][0], PropertyValue::String("core".into()));
    }

    #[test]
    fn run_match_return_graph() {
        let mut g = mini_graph();
        let result = run(&mut g, "MATCH (a:Piece)-[r:Connection]->(b:Piece) RETURN a, r, b").unwrap();
        assert_eq!(result.kind, QueryResultKind::Graph);
        let fixture = result.graph_fixture.expect("graph fixture");
        assert_eq!(fixture.nodes.len(), 2);
        assert_eq!(fixture.edges.len(), 1);
    }

    #[test]
    fn run_create() {
        let mut g = mini_graph();
        run(&mut g, "CREATE (n:Piece)").unwrap();
        assert_eq!(g.nodes.len(), 3);
    }

    #[test]
    fn run_set() {
        let mut g = mini_graph();
        run(&mut g, "MATCH (a:Piece) WHERE a.name = 'core' SET a.label = 'root-core'").unwrap();
        let node = g.node("root").unwrap();
        assert_eq!(node.properties.get("label"), Some(&PropertyValue::String("root-core".into())));
    }

    #[test]
    fn tokenize_keywords_and_strings() {
        let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core'");
        assert!(spans.iter().any(|s| s.class == TokenClass::Keyword && s.start == 0));
        assert!(spans.iter().any(|s| s.class == TokenClass::String));
    }

    #[test]
    fn tokenize_unterminated_string_is_error() {
        let spans = tokenize("MATCH (a:Piece) WHERE a.name = 'core");
        assert!(spans.iter().any(|s| s.class == TokenClass::Error));
    }

    #[test]
    fn complete_clause_keywords() {
        let g = mini_graph();
        let items = complete(&g, "MAT", 3);
        assert!(items.iter().any(|row| row.label == "MATCH"));
    }

    #[test]
    fn complete_node_kinds_after_colon() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:P", 11);
        assert!(items.iter().any(|row| row.label == "Piece"));
    }

    #[test]
    fn complete_properties_after_dot() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:Piece) WHERE a.n", 25);
        assert!(items.iter().any(|row| row.label == "name"));
    }

    #[test]
    fn complete_bound_variables() {
        let g = mini_graph();
        let items = complete(&g, "MATCH (a:Piece) RETURN a", 24);
        assert!(items.iter().any(|row| row.label == "a"));
    }

    #[test]
    fn lint_unterminated_string() {
        let g = mini_graph();
        let diags = lint(&g, "MATCH (a:Piece) WHERE a.name = 'core");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unterminated-string")));
    }

    #[test]
    fn lint_unbound_variable() {
        let g = mini_graph();
        let diags = lint(&g, "RETURN a.name");
        assert!(diags.iter().any(|d| d.code.as_deref() == Some("jack/unbound-variable")));
    }

    #[test]
    fn format_is_idempotent() {
        let source = "MATCH (a:Piece)--[r:Connection]->(b:Piece) RETURN a.name, b.name";
        let once = format_source(source).unwrap();
        let twice = format_source(&once).unwrap();
        assert_eq!(once, twice);
        assert!(once.contains("MATCH"));
        assert!(once.contains('\n'));
    }

    #[test]
    fn hover_keyword() {
        let g = mini_graph();
        let info = hover(&g, "MATCH (a:Piece) RETURN a.name", 2).unwrap();
        assert!(info.contents.contains("MATCH"));
    }

    #[test]
    fn semantic_tokens_cover_keywords() {
        let tokens = semantic_tokens("MATCH (a:Piece) RETURN a.name");
        assert!(tokens.iter().any(|t| t.class == "keyword"));
        assert!(tokens.iter().any(|t| t.class == "ident"));
    }

    #[test]
    fn run_create_edge() {
        let mut g = mini_graph();
        while g.nodes.len() < 9 {
            run(&mut g, "CREATE (n:Piece)").unwrap();
        }
        run(&mut g, "CREATE (x:Piece)-[:Connection]->(y:Piece)").unwrap();
        assert_eq!(g.nodes.len(), 11);
        assert_eq!(g.edges.len(), 2);
    }

    #[test]
    fn run_delete() {
        let mut g = mini_graph();
        run(&mut g, "MATCH (n:Piece) WHERE n.name = 'capsule' DELETE n").unwrap();
        assert_eq!(g.nodes.len(), 1);
        assert_eq!(g.edges.len(), 0);
    }

    #[test]
    fn run_merge_noop_when_pattern_exists() {
        let mut g = mini_graph();
        run(&mut g, "MERGE (a:Piece)-[:Connection]->(b:Piece)").unwrap();
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.edges.len(), 1);
    }

    #[test]
    fn run_merge_creates_disconnected_pattern() {
        let mut g = mini_graph();
        g.edges.clear();
        run(&mut g, "MERGE (x:Piece)-[:Connection]->(y:Piece)").unwrap();
        assert_eq!(g.nodes.len(), 4);
        assert_eq!(g.edges.len(), 1);
    }

    #[test]
    fn lex_not_equal() {
        let tokens = lex("WHERE a.name != 'core'").unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::Ne)));
    }
}
// #endregion 🔖️Tests