//! 🧮️ Trinity jack query executor.
#![allow(dead_code)]

use crate::standards::v1::subsets::any::schema::graph::{Edge, EntityRef, GraphEffect, Node};
use crate::standards::v1::subsets::any::schema::memory::Graph;
use semio_framework_graph_manifest::PropertyValue;
use std::collections::BTreeMap;

use crate::standards::v1::subsets::any::schema::ast::{Clause, Expr, Pattern, Query, QueryResult, ReturnItem};

/// 🎯️ Variable binding in a match row.
#[derive(Clone, Debug, Default)]
pub struct Binding {
    pub nodes: BTreeMap<String, String>,
    pub edges: BTreeMap<String, String>,
}

#[path = "🪜️execution/🦀️.rs"]
pub mod execution;
pub use execution::{QueryExecution, QueryExecutionPreparation, QueryPreparationStep, QUERY_RESULT_MAXIMUM_OWNED_BYTES};
/// ▶️ Runs the same evaluator used by retained artifact query jobs.
pub fn execute(graph: &Graph, query: &Query) -> Result<(QueryResult, Vec<GraphEffect>), String> {
    let mut execution = QueryExecution::new(graph.clone(), query.clone());
    loop {
        if let Some(result) = execution.step().map_err(|error| error.into_message())? {
            return Ok(result);
        }
    }
}

pub(crate) fn eval_expr(graph: &Graph, binding: &Binding, expr: &Expr) -> bool {
    match expr {
        Expr::Eq { var, prop, value } => binding_value(graph, binding, var, prop) == Some(value.clone()),
        Expr::Ne { var, prop, value } => binding_value(graph, binding, var, prop) != Some(value.clone()),
        Expr::And(a, b) => eval_expr(graph, binding, a) && eval_expr(graph, binding, b),
        Expr::Or(a, b) => eval_expr(graph, binding, a) || eval_expr(graph, binding, b),
    }
}

pub(crate) fn binding_value(graph: &Graph, binding: &Binding, var: &str, prop: &str) -> Option<PropertyValue> {
    if let Some(id) = binding.nodes.get(var) {
        let node = graph.node(id)?;
        return match prop {
            "id" => Some(PropertyValue::String(node.id.clone())),
            "name" => Some(PropertyValue::String(node.name.clone())),
            "kind" => Some(PropertyValue::String(node.kind.clone())),
            "x" => Some(PropertyValue::Number(node.x)),
            "y" => Some(PropertyValue::Number(node.y)),
            _ => node.properties.get(prop).cloned(),
        };
    }
    let edge = graph.edge(binding.edges.get(var)?)?;
    match prop {
        "id" => Some(PropertyValue::String(edge.id.clone())),
        "kind" => Some(PropertyValue::String(edge.kind.clone())),
        "source" => Some(PropertyValue::String(edge.source.clone())),
        "target" => Some(PropertyValue::String(edge.target.clone())),
        _ => edge.properties.get(prop).cloned(),
    }
}

pub(crate) fn binding_has_entity(graph: &Graph, binding: &Binding, var: &str) -> bool {
    binding.nodes.get(var).is_some_and(|id| graph.nodes.contains_key(id))
        || binding.edges.get(var).is_some_and(|id| graph.edges.contains_key(id))
}

#[cfg(any(test, feature = "testkit"))]
pub use execution::{node_owned_bytes as fixture_node_owned_bytes, edge_owned_bytes as fixture_edge_owned_bytes};
