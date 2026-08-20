//! 🔺️ `remove-node-property` — sparse diff construction; Error `mutation.target-missing` when the
//! owning BASE `node_id` is absent OR `index` is out of range (no property there to remove).

use super::mutation::RemoveNodeProperty;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff(payload: &RemoveNodeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if payload.index >= node.properties.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no property at index {}.", payload.node_id.value, payload.index), [payload.node_id.value.clone(), payload.index.to_string()]);
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.node_id).expect("checked above");
    node.properties.remove(payload.index);
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
