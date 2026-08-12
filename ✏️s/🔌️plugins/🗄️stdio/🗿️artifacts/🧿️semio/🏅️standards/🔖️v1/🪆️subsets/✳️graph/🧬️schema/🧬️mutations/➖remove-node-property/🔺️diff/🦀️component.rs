//! 🔺️ `remove-node-property` — sparse diff construction; an out-of-range BASE `node_id`/`index`
//! is a no-op clone.

use super::mutation::RemoveNodeProperty;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &RemoveNodeProperty, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.node_id) {
        if payload.index < node.properties.len() {
            node.properties.remove(payload.index);
        }
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
