//! ⚡️ Jack executor: matches a parsed query against a queryable graph and shapes the result.

use super::*;

// #region 🔖️Executor
/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
    /// 🍇️ Scalar/list/object values bound by `WITH`'s property projection, `UNWIND`, and `CALL` —
    /// entries here have no backing graph entity, unlike `nodes`/`edges`.
    pub values: BTreeMap<String, PropertyValue>,
}

/// ▶️ Execute a read-only jack query against a queryable graph.
pub fn execute<G: QueryableGraph>(graph: &G, query: &Query) -> Result<QueryResult, GraphDslError> {
    let mut bindings: Vec<Binding> = vec![Binding::default()];
    let mut return_items: Option<Vec<ReturnItem>> = None;
    for clause in &query.clauses {
        match clause {
            Clause::Match(patterns) => bindings = match_patterns(graph, patterns)?,
            Clause::Where(expr) => {
                let mut kept = Vec::with_capacity(bindings.len());
                for b in bindings {
                    if eval_expr(graph, &b, expr) {
                        kept.push(b);
                    }
                }
                bindings = kept;
            }
            Clause::Return(items) => return_items = Some(items.clone()),
            Clause::Create(_) | Clause::Delete(_) | Clause::Set(_) | Clause::Merge(_) => {
                return Err(GraphDslError::UnsupportedMutation);
            }
            Clause::With(items) => {
                bindings = bindings.iter().map(|binding| project_binding(binding, items)).collect();
            }
            Clause::Unwind(unwind) => {
                let mut next = Vec::new();
                for binding in &bindings {
                    for element in unwind_elements(resolve_item_value(graph, binding, &unwind.source)) {
                        let mut b = binding.clone();
                        b.values.insert(unwind.var.clone(), element);
                        next.push(b);
                    }
                }
                bindings = next;
            }
            Clause::Call(call) => {
                let (column, results) = call_procedure(graph, call)?;
                let mut next = Vec::with_capacity(bindings.len() * results.len());
                for binding in &bindings {
                    for value in &results {
                        let mut b = binding.clone();
                        b.values.insert(column.clone(), value.clone());
                        next.push(b);
                    }
                }
                bindings = next;
            }
        }
    }
    if let Some(items) = return_items {
        return Ok(build_return(graph, &bindings, &items));
    }
    Ok(QueryResult::table(vec![], vec![]))
}

/// ▶️ Parse and execute jack in one step.
pub fn run_query<G: QueryableGraph>(graph: &G, source: &str) -> Result<QueryResult, GraphDslError> {
    execute(graph, &parse(source)?)
}

/// ▶️ Execute jack and return JSON result.
pub fn run_query_json<G: QueryableGraph>(graph: &G, source: &str) -> Result<String, GraphDslError> {
    Ok(semio_framework_pack_json::to_json_string(&run_query(graph, source)?))
}

pub(crate) fn match_patterns<G: QueryableGraph>(graph: &G, patterns: &[Pattern]) -> Result<Vec<Binding>, GraphDslError> {
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

fn match_pattern<G: QueryableGraph>(graph: &G, pattern: &Pattern, base: &Binding) -> Result<Vec<Binding>, GraphDslError> {
    let left = pattern.nodes.first().ok_or(GraphDslError::EmptyPattern)?;
    if let Some(edge_pat) = &pattern.edge {
        let mut out = Vec::new();
        for node_id in graph.node_ids() {
            if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
                continue;
            }
            if binding_conflicts(base, &left.var, node_id.as_str()) {
                continue;
            }
            for edge in graph.edges() {
                if edge_pat.kind.as_ref().is_some_and(|k| *k != edge.kind) {
                    continue;
                }
                let pairs = if edge_pat.directed {
                    vec![(edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref())]
                } else {
                    vec![
                        (edge.source_node_id.as_str(), edge.target_node_id.as_str(), edge.source_port.as_deref(), edge.target_port.as_deref()),
                        (edge.target_node_id.as_str(), edge.source_node_id.as_str(), edge.target_port.as_deref(), edge.source_port.as_deref()),
                    ]
                };
                for (src_id, tgt_id, src_port, tgt_port) in pairs {
                    if src_id != node_id {
                        continue;
                    }
                    if left.port.as_ref().is_some_and(|want| src_port != Some(want.as_str())) {
                        continue;
                    }
                    if graph.node_kind(tgt_id).as_deref() != Some(edge_pat.right.kind.as_str()) {
                        continue;
                    }
                    if edge_pat.right.port.as_ref().is_some_and(|want| tgt_port != Some(want.as_str())) {
                        continue;
                    }
                    let mut b = base.clone();
                    b.nodes.insert(left.var.clone(), node_id.clone());
                    if let Some(ev) = &edge_pat.var {
                        b.edges.insert(ev.clone(), edge.id.clone());
                    }
                    if binding_conflicts(base, &edge_pat.right.var, tgt_id) {
                        continue;
                    }
                    b.nodes.insert(edge_pat.right.var.clone(), tgt_id.to_string());
                    out.push(b);
                }
            }
        }
        return Ok(out);
    }
    let mut out = Vec::new();
    for node_id in graph.node_ids() {
        if graph.node_kind(node_id.as_str()).as_deref() != Some(left.kind.as_str()) {
            continue;
        }
        if binding_conflicts(base, &left.var, node_id.as_str()) {
            continue;
        }
        let mut b = base.clone();
        b.nodes.insert(left.var.clone(), node_id);
        out.push(b);
    }
    Ok(out)
}

fn binding_conflicts(base: &Binding, var: &str, node_id: &str) -> bool {
    base.nodes.get(var).is_some_and(|existing| existing != node_id)
}

fn eval_expr<G: QueryableGraph>(graph: &G, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

fn binding_value<G: QueryableGraph>(graph: &G, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    if let Some(node_id) = binding.nodes.get(var) {
        return graph.node_property(node_id, prop);
    }
    match binding.values.get(var) {
        Some(PropertyValue::Object(map)) => map.get(prop).cloned(),
        _ => None,
    }
}

/// 🎯️ Resolves a [`ReturnItem`] against one binding — the single evaluation path shared by
/// `RETURN`, `WITH`'s trailing `WHERE`, and `UNWIND`'s source expression. A `Var` prefers a
/// `values`-scoped scalar (bound by `WITH`/`UNWIND`/`CALL`) over a graph entity's display name.
fn resolve_item_value<G: QueryableGraph>(graph: &G, binding: &Binding, item: &ReturnItem) -> Option<PropertyValue> {
    match item {
        ReturnItem::Var(v) => {
            if let Some(value) = binding.values.get(v) {
                return Some(value.clone());
            }
            binding.nodes.get(v).map(|id| graph.node_name(id).map_or(PropertyValue::Null, PropertyValue::String))
        }
        ReturnItem::Property { var, prop } => binding_value(graph, binding, var, prop),
    }
}

/// 🧺️ `UNWIND`'s list-expansion rule: `null`/missing and an empty list both yield zero rows, a
/// non-list scalar unwinds as its own single-element list (mirrors the Cypher-family convention).
fn unwind_elements(value: Option<PropertyValue>) -> Vec<PropertyValue> {
    match value {
        Some(PropertyValue::Array(items)) => items,
        Some(PropertyValue::Null) | None => vec![],
        Some(other) => vec![other],
    }
}

/// 🔭️ Projects one binding down to just the vars named by a `WITH` clause. A `Property` item
/// names its source var (`ReturnItem` carries no alias), so the whole entity/value bound to that
/// var is kept — this is what lets a later `WHERE`/`RETURN` still resolve `var.prop` from it.
fn project_binding(binding: &Binding, items: &[ReturnItem]) -> Binding {
    let mut out = Binding::default();
    for item in items {
        let var = match item {
            ReturnItem::Var(v) => v,
            ReturnItem::Property { var, .. } => var,
        };
        if let Some(id) = binding.nodes.get(var) {
            out.nodes.insert(var.clone(), id.clone());
        }
        if let Some(id) = binding.edges.get(var) {
            out.edges.insert(var.clone(), id.clone());
        }
        if let Some(value) = binding.values.get(var) {
            out.values.insert(var.clone(), value.clone());
        }
    }
    out
}

/// 📇️ `CALL`'s small owned procedure registry — graph-manifest introspection this codebase
/// already exposes to completion/hover (`manifest_node_kinds` et al.), surfaced as callable
/// procedures since `CallClause` has no `YIELD`/`AS` target: each procedure's single output
/// column is bound under a fixed, procedure-defined name. Arbitrary procedure dispatch (the
/// general case) is deferred to unifying semio_compose_rs's Architect query language onto Jack —
/// an unknown name reports [`GraphDslError::UnknownProcedure`], never a blanket rejection.
fn call_procedure<G: QueryableGraph>(graph: &G, call: &CallClause) -> Result<(String, Vec<PropertyValue>), GraphDslError> {
    if !call.args.is_empty() {
        return Err(GraphDslError::ProcedureArity { name: call.name.clone(), expected: 0, found: call.args.len() });
    }
    match call.name.as_str() {
        "nodeKinds" => Ok(("kind".to_string(), manifest_node_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "edgeKinds" => Ok(("kind".to_string(), manifest_edge_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        "propertyNames" => Ok(("name".to_string(), manifest_property_names(graph).into_iter().map(PropertyValue::String).collect())),
        "portKinds" => Ok(("kind".to_string(), manifest_port_kinds(graph).into_iter().map(PropertyValue::String).collect())),
        other => Err(GraphDslError::UnknownProcedure(other.to_string())),
    }
}

fn binding_has_entity(binding: &Binding, var: &str) -> bool {
    binding.nodes.contains_key(var) || binding.edges.contains_key(var)
}

fn return_items_want_graph(items: &[ReturnItem], bindings: &[Binding]) -> bool {
    for item in items {
        let ReturnItem::Var(v) = item else { continue };
        for b in bindings {
            if binding_has_entity(b, v) {
                return true;
            }
        }
    }
    false
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

fn build_return<G: QueryableGraph>(graph: &G, bindings: &[Binding], items: &[ReturnItem]) -> QueryResult {
    let columns: Vec<String> = items
        .iter()
        .map(|item| match item {
            ReturnItem::Var(v) => v.clone(),
            ReturnItem::Property { var, prop } => format!("{var}.{prop}"),
        })
        .collect();
    if return_items_want_graph(items, bindings) {
        let (node_ids, edge_ids) = collect_graph_entities(bindings, items);
        if let Some(json) = graph.subgraph_snapshot_json(&node_ids, &edge_ids) {
            return QueryResult::graph(columns, json);
        }
    }
    let mut rows = Vec::new();
    for binding in bindings {
        let mut row = Vec::new();
        for item in items {
            row.push(resolve_item_value(graph, binding, item).unwrap_or(PropertyValue::Null));
        }
        rows.push(row);
    }
    QueryResult::table(columns, rows)
}
// #endregion 🔖️Executor
