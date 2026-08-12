//! 🔺️ `add-node-property` — sparse diff construction; an out-of-range BASE `node_id` is a no-op
//! clone (nothing at that position to attach a property to).

use super::mutation::AddNodeProperty;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &AddNodeProperty, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.node_id) {
        let at = payload.index.min(node.properties.len());
        node.properties.insert(at, payload.property.clone());
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
