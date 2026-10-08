//! 🧮️ Net of one snapshot edit as flow domain leaves: the delta between the edited snapshot and the base, expressed as the
//! concrete `flow` mutations that carry the base to it. A change the vocabulary cannot express yields leaves whose fold differs from
//! the edit, which the editor's publication check refuses.

use crate::standards::v1::subsets::base::schema::triples::net_keyed;
use crate::standards::v1::subsets::flow::schema::mutations::{insert_edge, insert_node, remove_edge, remove_node, remove_node_param, set_edge_endpoints, set_edge_kind, set_node_kind, set_node_label, set_node_param, set_node_position, SemioFlowMutation};
use crate::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn net(base: &SemioFlowSnapshot, next: &SemioFlowSnapshot) -> Vec<SemioFlowMutation> {
    let nodes = net_keyed(&base.nodes, &next.nodes, |node| node.id.clone());
    let edges = net_keyed(&base.edges, &next.edges, |edge| edge.id.clone());
    let mut out = Vec::new();
    out.extend(edges.removed.iter().map(|edge| SemioFlowMutation::RemoveEdge(remove_edge::RemoveEdge { id: edge.id.clone() })));
    out.extend(nodes.removed.iter().map(|node| SemioFlowMutation::RemoveNode(remove_node::RemoveNode { id: node.id.clone() })));
    for (before, after) in &nodes.modified {
        let id = after.id.clone();
        if before.kind != after.kind {
            out.push(SemioFlowMutation::SetNodeKind(set_node_kind::SetNodeKind { id: id.clone(), kind: after.kind.clone() }));
        }
        if before.label != after.label {
            out.push(SemioFlowMutation::SetNodeLabel(set_node_label::SetNodeLabel { id: id.clone(), label: after.label.clone() }));
        }
        if before.position != after.position {
            out.push(SemioFlowMutation::SetNodePosition(set_node_position::SetNodePosition { id: id.clone(), position: after.position }));
        }
        for param in before.params.iter().filter(|param| !after.params.iter().any(|other| other.key == param.key)) {
            out.push(SemioFlowMutation::RemoveNodeParam(remove_node_param::RemoveNodeParam { id: id.clone(), key: param.key.clone() }));
        }
        for param in after.params.iter().filter(|param| !before.params.iter().any(|other| other.key == param.key && other.value == param.value)) {
            out.push(SemioFlowMutation::SetNodeParam(set_node_param::SetNodeParam { id: id.clone(), key: param.key.clone(), value: param.value.clone(), at: None }));
        }
    }
    out.extend(nodes.added.iter().map(|node| SemioFlowMutation::InsertNode(insert_node::InsertNode { node: (*node).clone(), at: None })));
    out.extend(edges.added.iter().map(|edge| SemioFlowMutation::InsertEdge(insert_edge::InsertEdge { edge: (*edge).clone(), at: None })));
    for (before, after) in &edges.modified {
        if before.from != after.from || before.to != after.to {
            out.push(SemioFlowMutation::SetEdgeEndpoints(set_edge_endpoints::SetEdgeEndpoints { id: after.id.clone(), from: after.from.clone(), to: after.to.clone() }));
        }
        if before.kind != after.kind {
            out.push(SemioFlowMutation::SetEdgeKind(set_edge_kind::SetEdgeKind { id: after.id.clone(), kind: after.kind.clone() }));
        }
    }
    out
}
