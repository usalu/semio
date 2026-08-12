//! 🔺️ `remove-node-port` — sparse diff construction; an out-of-range BASE `node_id`/`index` is a
//! no-op clone.

use super::mutation::RemoveNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveNodePort, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.node_id) {
        if payload.index < node.ports.len() {
            node.ports.remove(payload.index);
        }
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
