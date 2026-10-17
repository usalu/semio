//! 🧰️ `SemioFlowMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::FlowEdge;
use crate::schema::snapshot::FlowNode;
use crate::schema::snapshot::SemioFlowSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn node_at<'a>(base: &'a SemioFlowSnapshot, id: &str) -> Option<&'a FlowNode> {
    base.nodes.iter().find(|n| n.id == id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn edge_at<'a>(base: &'a SemioFlowSnapshot, id: &str) -> Option<&'a FlowEdge> {
    base.edges.iter().find(|e| e.id == id)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn param_value_at<'a>(base: &'a SemioFlowSnapshot, id: &str, key: &str) -> Option<&'a str> {
    node_at(base, id)?.params.iter().find(|p| p.key == key).map(|p| p.value.as_str())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn param_index_at(base: &SemioFlowSnapshot, id: &str, key: &str) -> Option<usize> {
    node_at(base, id)?.params.iter().position(|p| p.key == key)
}
