//! 🔺️ Diff for `RemoveNodePort`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::RemoveNodePort, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if payload.index >= node.ports.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no port at index {}.", payload.node_id.value, payload.index), [payload.node_id.value.clone(), payload.index.to_string()]);
    }
    let at = base.nodes.iter().position(|n| n.id == payload.node_id).expect("checked above");
    let ports = IndexedTripleDiff { removed: vec![payload.index], ..Default::default() };
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { ports: Some(ports), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
