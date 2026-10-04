//! 🔺️ Diff for `SetNodeProperty`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

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
    let mut nodes = base.nodes.clone();
    if let Some(property) = nodes.iter_mut().filter(|node| node.id == payload.node_id).flat_map(|node| node.properties.iter_mut()).find(|property| property.key == payload.key) {
        property.value = payload.value.clone();
    }
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
