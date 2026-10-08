//! 🔺️ Diff for `AddNodeProperty`.

use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &super::AddNodeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if node.properties.iter().any(|p| p.key == payload.property.key) {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Node \"{}\" already has a property \"{}\".", payload.node_id.value, payload.property.key));
    }
    let at = base.nodes.iter().position(|row| row.id == payload.node_id).expect("checked above");
    let properties = IndexedTripleDiff { added: vec![IndexAdded { index: payload.index.min(base.nodes[at].properties.len()), item: payload.property.clone() }], ..Default::default() };
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { properties: Some(properties), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
