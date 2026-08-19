//! 🔺️ `remove-node-port` — sparse diff construction; Error `mutation.target-missing` when the
//! owning BASE `node_id` is absent OR `index` is out of range (no port there to remove).

use super::mutation::RemoveNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &RemoveNodePort, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if payload.index >= node.ports.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" has no port at index {}.", payload.node_id.value, payload.index), [payload.node_id.value.clone(), payload.index.to_string()]);
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.node_id).expect("checked above");
    node.ports.remove(payload.index);
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
