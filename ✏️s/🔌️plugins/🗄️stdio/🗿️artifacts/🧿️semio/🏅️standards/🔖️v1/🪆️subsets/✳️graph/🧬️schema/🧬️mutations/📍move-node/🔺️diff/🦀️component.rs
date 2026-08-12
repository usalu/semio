//! 🔺️ `move-node` — sparse diff construction; an out-of-range BASE `id` is a no-op clone.

use super::mutation::MoveNode;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &MoveNode, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.id) {
        node.position = payload.new_position.clone();
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
