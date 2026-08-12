//! 🔺️ `change-node-label` — sparse diff construction; an out-of-range BASE `id` is a no-op clone.

use super::mutation::ChangeNodeLabel;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNodeLabel, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.id) {
        node.label = payload.new_label.clone();
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
