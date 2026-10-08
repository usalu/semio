//! 🧮️ Net of one snapshot edit as graph domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `graph` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::{net_keyed, net_ordered, NetStep};
use crate::standards::v1::subsets::graph::schema::mutations::{
    add_edge_property, add_node_port, add_node_property, change_node_kind, change_node_label, create_edge, create_node, delete_edge, delete_node, move_node, remove_edge_property, remove_node_port, remove_node_property, resize_node, set_edge_property, set_node_property,
    SemioGraphMutation,
};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphEdge, SemioGraphNode, SemioGraphSnapshot};
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueEntry;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioGraphSnapshot, next: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    let nodes = net_keyed(&base.nodes, &next.nodes, |node| node.id.clone());
    let edges = net_keyed(&base.edges, &next.edges, |edge| edge.id.clone());
    let mut out = Vec::new();
    out.extend(edges.removed.iter().map(|edge| SemioGraphMutation::DeleteEdge(delete_edge::DeleteEdge { id: edge.id.clone() })));
    out.extend(nodes.removed.iter().map(|node| SemioGraphMutation::DeleteNode(delete_node::DeleteNode { id: node.id.clone() })));
    for node in &nodes.added {
        let at = next.nodes.iter().position(|other| other.id == node.id);
        out.push(SemioGraphMutation::CreateNode(create_node::CreateNode {
            id: node.id.clone(),
            kind: node.kind.clone(),
            label: node.label.clone(),
            position: node.position,
            width: node.width,
            height: node.height,
            ports: node.ports.clone(),
            properties: node.properties.clone(),
            at,
        }));
    }
    for (before, after) in &nodes.modified {
        net_node(before, after, &mut out);
    }
    for edge in &edges.added {
        let at = next.edges.iter().position(|other| other.id == edge.id);
        out.push(SemioGraphMutation::CreateEdge(create_edge::CreateEdge {
            id: edge.id.clone(),
            source: edge.source.clone(),
            target: edge.target.clone(),
            kind: edge.kind.clone(),
            label: edge.label.clone(),
            source_port: edge.source_port.clone(),
            target_port: edge.target_port.clone(),
            properties: edge.properties.clone(),
            at,
        }));
    }
    for (before, after) in &edges.modified {
        net_properties(&before.properties, &after.properties, &mut |key| remove_edge(after, key), &mut |index, property| add_edge(after, index, property), &mut |entry| set_edge(after, entry), &mut out);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn remove_edge(edge: &SemioGraphEdge, key: &str) -> SemioGraphMutation {
    SemioGraphMutation::RemoveEdgeProperty(remove_edge_property::RemoveEdgeProperty { edge_id: edge.id.clone(), key: key.to_string() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn add_edge(edge: &SemioGraphEdge, index: usize, property: &SemioValueEntry) -> SemioGraphMutation {
    SemioGraphMutation::AddEdgeProperty(add_edge_property::AddEdgeProperty { edge_id: edge.id.clone(), index, property: property.clone() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn set_edge(edge: &SemioGraphEdge, entry: &SemioValueEntry) -> SemioGraphMutation {
    SemioGraphMutation::SetEdgeProperty(set_edge_property::SetEdgeProperty { edge_id: edge.id.clone(), key: entry.key.clone(), value: entry.value.clone() })
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_properties(
    before: &[SemioValueEntry],
    after: &[SemioValueEntry],
    remove: &mut dyn FnMut(&str) -> SemioGraphMutation,
    add: &mut dyn FnMut(usize, &SemioValueEntry) -> SemioGraphMutation,
    set: &mut dyn FnMut(&SemioValueEntry) -> SemioGraphMutation,
    out: &mut Vec<SemioGraphMutation>,
) {
    let keyed = net_keyed(before, after, |entry| entry.key.clone());
    out.extend(keyed.removed.iter().map(|entry| remove(&entry.key)));
    out.extend(keyed.modified.iter().map(|(_, entry)| set(entry)));
    for entry in keyed.added {
        let index = after.iter().position(|other| other.key == entry.key).unwrap_or(after.len());
        out.push(add(index, entry));
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn net_node(before: &SemioGraphNode, after: &SemioGraphNode, out: &mut Vec<SemioGraphMutation>) {
    let id = after.id.clone();
    if before.kind != after.kind {
        out.push(SemioGraphMutation::ChangeNodeKind(change_node_kind::ChangeNodeKind { id: id.clone(), new_kind: after.kind.clone() }));
    }
    if before.label != after.label {
        out.push(SemioGraphMutation::ChangeNodeLabel(change_node_label::ChangeNodeLabel { id: id.clone(), new_label: after.label.clone() }));
    }
    if before.position != after.position {
        out.push(SemioGraphMutation::MoveNode(move_node::MoveNode { id: id.clone(), new_position: after.position }));
    }
    if (before.width, before.height) != (after.width, after.height) {
        out.push(SemioGraphMutation::ResizeNode(resize_node::ResizeNode { id: id.clone(), width: after.width, height: after.height }));
    }
    for step in net_ordered(&before.ports, &after.ports) {
        match step {
            NetStep::Modify { index, item } => {
                out.push(SemioGraphMutation::RemoveNodePort(remove_node_port::RemoveNodePort { node_id: id.clone(), index }));
                out.push(SemioGraphMutation::AddNodePort(add_node_port::AddNodePort { node_id: id.clone(), index, port: item.clone() }));
            }
            NetStep::Remove { index } => out.push(SemioGraphMutation::RemoveNodePort(remove_node_port::RemoveNodePort { node_id: id.clone(), index })),
            NetStep::Insert { index, item } => out.push(SemioGraphMutation::AddNodePort(add_node_port::AddNodePort { node_id: id.clone(), index, port: item.clone() })),
        }
    }
    net_properties(
        &before.properties,
        &after.properties,
        &mut |key| SemioGraphMutation::RemoveNodeProperty(remove_node_property::RemoveNodeProperty { node_id: id.clone(), key: key.to_string() }),
        &mut |index, property| SemioGraphMutation::AddNodeProperty(add_node_property::AddNodeProperty { node_id: id.clone(), index, property: property.clone() }),
        &mut |entry| SemioGraphMutation::SetNodeProperty(set_node_property::SetNodeProperty { node_id: id.clone(), key: entry.key.clone(), value: entry.value.clone() }),
        out,
    );
}
