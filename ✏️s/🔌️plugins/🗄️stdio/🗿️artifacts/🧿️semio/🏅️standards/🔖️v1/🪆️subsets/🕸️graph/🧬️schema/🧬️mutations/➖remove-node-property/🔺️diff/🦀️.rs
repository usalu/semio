//! 🔺️ Diff for `RemoveNodeProperty`.

use crate::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
/// 🧮️ A node the graph lacks, or a key it does not carry, is `mutation.target-missing`; otherwise that entry leaves.
pub fn diff(payload: &super::RemoveNodeProperty, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(position) = base.nodes.iter().position(|node| node.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    let Some(index) = base.nodes[position].properties.iter().position(|property| property.key == payload.key) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no property \"{}\".", payload.node_id.value, payload.key), [payload.node_id.value.clone(), payload.key.clone()]);
    };
    let mut nodes = base.nodes.clone();
    nodes[position].properties.remove(index);
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
