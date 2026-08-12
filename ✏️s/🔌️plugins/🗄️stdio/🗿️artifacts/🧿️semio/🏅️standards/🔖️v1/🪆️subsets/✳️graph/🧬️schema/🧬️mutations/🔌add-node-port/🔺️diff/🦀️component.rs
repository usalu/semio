//! 🔺️ `add-node-port` — sparse diff construction; an out-of-range BASE `node_id` is a no-op clone
//! (nothing at that position to attach a port to).

use super::mutation::AddNodePort;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddNodePort, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.node_id) {
        let at = payload.index.min(node.ports.len());
        node.ports.insert(at, payload.port.clone());
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
