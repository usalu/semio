//! 🔺️ Diff for `AddNodePort`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddNodePort, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if node.ports.iter().any(|p| p.name == payload.port.name) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has a port \"{}\".", payload.node_id.value, payload.port.name));
    }
    let at = base.nodes.iter().position(|n| n.id == payload.node_id).expect("checked above");
    let ports = IndexedTripleDiff { added: vec![IndexAdded { index: payload.index.min(base.nodes[at].ports.len()), item: payload.port.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { ports: Some(ports), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
