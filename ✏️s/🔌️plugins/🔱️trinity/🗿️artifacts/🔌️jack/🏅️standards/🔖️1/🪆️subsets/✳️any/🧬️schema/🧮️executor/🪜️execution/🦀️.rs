//! 🪜️ Query execution retains one explicit preparation, evaluation, result, or retirement cursor.

use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::any::schema::host::{admit_demand, JackSnapshotCloneAuthority, JackSnapshotCloneStep};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::RetirementDemand;
use crate::standards::v1::subsets::any::schema::ast::PatternNode;
use crate::standards::v1::subsets::any::schema::graph::{port_key, port_node_id, Port};
use crate::standards::v1::subsets::any::schema::operations::validate_graph_effect;
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

fn invalid(message: impl Into<String>) -> ValueError {
    ValueError::new(ValueRefusalKind::InvalidValue, message)
}

fn endpoint_node(key: &str) -> &str {
    port_node_id(key).unwrap_or(key)
}

fn bound_entity(binding: &Binding, var: &str) -> Result<EntityRef, ValueError> {
    if let Some(id) = binding.nodes.get(var) {
        return Ok(EntityRef::Node(id.clone()));
    }
    binding.edges.get(var).cloned().map(EntityRef::Edge).ok_or_else(|| invalid(format!("unbound variable {var}")))
}

fn emit_set_operation_from_graph(graph: &Graph, entity: EntityRef, prop: &str, value: PropertyValue) -> Result<GraphEffect, ValueError> {
    let EntityRef::Node(node_id) = &entity else {
        return match prop {
            "id" | "kind" | "source" | "target" => Err(invalid(format!("edge property {prop} is read-only"))),
            _ => Ok(GraphEffect::SetProperty { entity, key: prop.to_string(), value }),
        };
    };
    let node = graph.nodes.get(node_id).ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id} not found")))?;
    match prop {
        "name" => match value {
            PropertyValue::String(name) => Ok(GraphEffect::RenameNode { id: node_id.to_string(), name }),
            _ => Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.name expects string value"))),
        },
        "x" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.x expects number value")))?, y: node.y }),
        "y" => Ok(GraphEffect::MoveNode { id: node_id.to_string(), x: node.x, y: value.as_f64().ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue,format!("node {node_id}.y expects number value")))? }),
        "id" | "kind" => Err(invalid(format!("node property {prop} is read-only"))),
        _ => Ok(GraphEffect::SetProperty { entity, key: prop.to_string(), value }),
    }
}

#[derive(Default)]
struct CreationIds {
    node: usize,
    edge: usize,
    deleted_nodes: BTreeSet<String>,
    deleted_edges: BTreeSet<String>,
}

fn available_id<T>(prefix: &str, serial: &mut usize, entries: &BTreeMap<String, T>, deleted: &BTreeSet<String>) -> String {
    *serial = (*serial).max(entries.len());
    loop {
        let id = format!("{prefix}-{serial}");
        *serial += 1;
        if !entries.contains_key(&id) && !deleted.contains(&id) {
            return id;
        }
    }
}

fn create_pattern_node(graph: &Graph, pattern: &PatternNode, binding: &Binding, directions: &[PortDirection], ids: &mut CreationIds, operations: &mut Vec<GraphEffect>) -> Result<String, ValueError> {
    if binding.edges.contains_key(&pattern.var) {
        return Err(invalid(format!("{} is already bound to an edge", pattern.var)));
    }
    let def = graph.manifest.node_kind(&pattern.kind).ok_or_else(|| invalid(format!("unknown node kind {:?}", pattern.kind)))?;
    if let Some(id) = binding.nodes.get(&pattern.var) {
        if !graph.node(id).is_some_and(|node| node.kind == pattern.kind) {
            return Err(invalid(format!("{} is not a live {} node", pattern.var, pattern.kind)));
        }
        return Ok(id.clone());
    }
    let id = available_id(&pattern.var, &mut ids.node, &graph.nodes, &ids.deleted_nodes);
    let ports = directions.iter().filter_map(|direction| {
        let compatible = |port: &&crate::standards::v1::subsets::any::schema::manifest::PortKindDef| {
            def.port_kinds.is_empty() || def.port_kinds.contains(&port.name)
        };
        let mut declared = graph.manifest.port_kinds.iter().filter(compatible);
        let kind = declared.clone().find(|port| port.direction == *direction)
            .or_else(|| declared.find(|port| port.name == "Connector"))?;
        let id = match direction {
            PortDirection::In => "in",
            PortDirection::Out => "out",
        };
        Some(Port { id: id.into(), kind: kind.name.clone(), direction: *direction, properties: PropertyBag::new() })
    }).collect();
    operations.push(GraphEffect::CreateNode(Node {
        id: id.clone(), kind: pattern.kind.clone(), name: pattern.var.clone(),
        x: (graph.nodes.len() + operations.len()) as f64 * 120.0, y: if directions == [PortDirection::In] { 80.0 } else { 0.0 },
        width: 80.0, height: 40.0, properties: PropertyBag::new(), ports,
    }));
    Ok(id)
}

fn creation_endpoint(graph: &Graph, operations: &[GraphEffect], id: &str, direction: PortDirection) -> String {
    let node = graph.node(id).or_else(|| operations.iter().find_map(|effect| match effect {
        GraphEffect::CreateNode(node) if node.id == id => Some(node),
        _ => None,
    }));
    node.and_then(|node| node.ports.iter().find(|port| port.direction == direction))
        .map_or_else(|| id.to_owned(), |port| port_key(id, &port.id))
}

fn create_operations(graph: &Graph, pattern: &Pattern, binding: &mut Binding, ids: &mut CreationIds) -> Result<Vec<GraphEffect>, ValueError> {
    let left = pattern.nodes.first().ok_or_else(|| invalid("empty create pattern"))?;
    let mut operations = Vec::new();
    let Some(edge) = &pattern.edge else {
        let id = create_pattern_node(graph, left, binding, &[], ids, &mut operations)?;
        binding.nodes.insert(left.var.clone(), id);
        return Ok(operations);
    };
    let kind = edge.kind.as_deref()
        .or_else(|| graph.manifest.edge_kind("Connection").map(|kind| kind.name.as_str()))
        .or_else(|| match graph.manifest.edge_kinds.as_slice() { [kind] => Some(kind.name.as_str()), _ => None })
        .ok_or_else(|| invalid("created edge requires an explicit kind"))?;
    if let Some(var) = &edge.var {
        if binding.nodes.contains_key(var) || binding.edges.contains_key(var) || *var == left.var || *var == edge.right.var {
            return Err(invalid(format!("{var} is already bound")));
        }
    }
    let (source, target) = if left.var == edge.right.var {
        if left.kind != edge.right.kind {
            return Err(invalid(format!("{} has conflicting node kinds", left.var)));
        }
        let id = create_pattern_node(graph, left, binding, &[PortDirection::Out, PortDirection::In], ids, &mut operations)?;
        (id.clone(), id)
    } else {
        (
            create_pattern_node(graph, left, binding, &[PortDirection::Out], ids, &mut operations)?,
            create_pattern_node(graph, &edge.right, binding, &[PortDirection::In], ids, &mut operations)?,
        )
    };
    let id = available_id("e", &mut ids.edge, &graph.edges, &ids.deleted_edges);
    operations.push(GraphEffect::CreateEdge(Edge {
        id: id.clone(), kind: kind.to_owned(),
        source: creation_endpoint(graph, &operations, &source, PortDirection::Out),
        target: creation_endpoint(graph, &operations, &target, PortDirection::In),
        properties: PropertyBag::new(),
    }));
    binding.nodes.insert(left.var.clone(), source);
    binding.nodes.insert(edge.right.var.clone(), target);
    if let Some(var) = &edge.var {
        binding.edges.insert(var.clone(), id);
    }
    Ok(operations)
}
