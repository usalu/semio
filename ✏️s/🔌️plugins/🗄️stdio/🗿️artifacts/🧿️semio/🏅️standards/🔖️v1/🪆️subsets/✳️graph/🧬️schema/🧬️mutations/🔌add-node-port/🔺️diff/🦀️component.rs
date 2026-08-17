//! 🔺️ `add-node-port` — sparse diff construction; Error `mutation.target-missing` when the owning
//! BASE `node_id` is absent, Warning `mutation.no-op` when the node already has a port with this
//! name.

use super::mutation::AddNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddNodePort, base: &SemioGraphSnapshot) -> protocol::MutationOutcome<SemioGraphDiff> {
    let Some(node) = base.nodes.iter().find(|n| n.id == payload.node_id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Node \"{}\" does not exist.", payload.node_id.value), [payload.node_id.value.clone()]);
    };
    if node.ports.iter().any(|p| p.name == payload.port.name) {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Node \"{}\" already has a port \"{}\".", payload.node_id.value, payload.port.name));
    }
    let mut nodes = base.nodes.clone();
    let node = nodes.iter_mut().find(|n| n.id == payload.node_id).expect("checked above");
    let at = payload.index.min(node.ports.len());
    node.ports.insert(at, payload.port.clone());
    protocol::MutationOutcome::new(SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None })
}
//#endregion 🔖️Diff
