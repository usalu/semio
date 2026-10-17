//! 🪜️ Query execution retains one explicit preparation, evaluation, result, or retirement cursor.

use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::any::schema::host::{admit_demand, JackSnapshotCloneAuthority, JackSnapshotCloneStep};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::RetirementDemand;
use crate::standards::v1::subsets::any::schema::graph::{port_key, Port};
use crate::standards::v1::subsets::any::schema::snapshot::JackSnapshot;
use semio_framework_graph_manifest::{PortDirection, PropertyBag};

use std::collections::{BTreeSet, VecDeque};
use std::ops::Bound::{Excluded, Unbounded};

#[path = "⚖️owned-bytes/🦀️.rs"]
mod owned_bytes;
pub use owned_bytes::*;

#[path = "🧱️execution-preparation/🦀️.rs"]
mod execution_preparation;
pub use execution_preparation::*;

#[path = "🔁️pattern-delete-return/🦀️.rs"]
mod pattern_delete_return;
use pattern_delete_return::*;

#[path = "▶️query-execution/🦀️.rs"]
mod query_execution;
pub use query_execution::*;

pub fn emit_set_operation_from_graph(graph: &Graph, node_id: &str, prop: &str, value: PropertyValue) -> Result<GraphEffect, ValueError> {
    let node = graph.nodes.get(node_id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id} not found")))?;
    match prop {
        "name" => match value {
            PropertyValue::String(name) => Ok(GraphEffect::RenameNode { id: node_id.to_string(), name }),
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.name expects string value"))),
        },
        "x" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.x expects number value")))?, y: node.y }),
        "y" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: node.x, y: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.y expects number value")))? }),
        _ => Ok(GraphEffect::SetProperty { entity: EntityRef::Node(node_id.to_string()), key: prop.to_string(), value }),
    }
}

pub fn emit_create_operations_from_graph(graph: &Graph, pattern: &Pattern) -> Result<Vec<GraphEffect>, ValueError> {
    let left = pattern.nodes.first().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,"empty create pattern"))?;
    let left_id = format!("{}-{}", left.var, graph.nodes.len());
    let mut operations = Vec::with_capacity(if pattern.edge.is_some() { 3 } else { 1 });
    let mut left_ports = Vec::new();
    if pattern.edge.is_some() {
        left_ports.push(Port { id: "out".into(), kind: "Connector".into(), direction: PortDirection::Out, properties: PropertyBag::new() });
    }
    operations.push(GraphEffect::CreateNode(Node { id: left_id.clone(), kind: left.kind.clone(), name: left.var.clone(), x: graph.nodes.len() as f64 * 120.0, y: 0.0, width: 80.0, height: 40.0, properties: PropertyBag::new(), ports: left_ports }));
    if let Some(edge_pattern) = &pattern.edge {
        let right_id = format!("{}-{}", edge_pattern.right.var, graph.nodes.len() + 1);
        operations.push(GraphEffect::CreateNode(Node {
            id: right_id.clone(),
            kind: edge_pattern.right.kind.clone(),
            name: edge_pattern.right.var.clone(),
            x: (graph.nodes.len() + 1) as f64 * 120.0,
            y: 80.0,
            width: 80.0,
            height: 40.0,
            properties: PropertyBag::new(),
            ports: vec![Port { id: "in".into(), kind: "Connector".into(), direction: PortDirection::In, properties: PropertyBag::new() }],
        }));
        operations.push(GraphEffect::CreateEdge(Edge {
            id: format!("e-{}", graph.edges.len()),
            kind: edge_pattern.kind.clone().unwrap_or_else(|| "Connection".into()),
            source: port_key(&left_id, "out"),
            target: port_key(&right_id, "in"),
            properties: PropertyBag::new(),
        }));
    }
    Ok(operations)
}
