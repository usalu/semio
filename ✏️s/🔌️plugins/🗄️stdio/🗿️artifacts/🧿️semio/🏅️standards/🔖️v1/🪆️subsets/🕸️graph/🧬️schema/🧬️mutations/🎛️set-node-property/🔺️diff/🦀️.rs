//! 🔺️ Diff for `SetNodeProperty`.

use crate::standards::v1::subsets::base::schema::triples::{IndexModified, IndexedTripleDiff};
use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphEntryDiff, SemioGraphNodeDiff};
use crate::standards::v1::subsets::graph::schema::snapshot::{SemioGraphSnapshot};

//#region 🔖️Diff
/// 🧮️ An empty key is a Fatal `mutation.invariant`; a node the graph lacks, or a key the node lacks, is
/// `mutation.target-missing`; a value equal to the current one is `mutation.no-op`; otherwise the property takes the value
/// in place (its position among the node's properties is kept).
pub fn diff(payload: &super::SetNodeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let target = [payload.node_id.value.clone()];
    if payload.key.is_empty() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a node property key is never empty", target);
    }
    let Some(node) = base.nodes.iter().find(|node| node.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), target);
    };
    let Some(current) = node.properties.iter().find(|property| property.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no property \"{}\".", payload.node_id.value, payload.key), target);
    };
    if current.value == payload.value {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Property \"{}\" of node \"{}\" already has that value.", payload.key, payload.node_id.value));
    }
    let at = base.nodes.iter().position(|row| row.id == payload.node_id).expect("checked above");
    let index = base.nodes[at].properties.iter().position(|property| property.key == payload.key).expect("checked above");
    let properties = IndexedTripleDiff { modified: vec![IndexModified { index, diff: SemioGraphEntryDiff { value: Some(payload.value.clone()) } }], ..Default::default() };
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(IndexedTripleDiff { modified: vec![IndexModified { index: at, diff: SemioGraphNodeDiff { properties: Some(properties), ..Default::default() } }], ..Default::default() }), edges: None })
}
//#endregion 🔖️Diff
