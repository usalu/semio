//! 🔺️ `change-node-kind` — sparse diff construction; an out-of-range BASE `id` is a no-op clone.

use super::mutation::ChangeNodeKind;
use crate::artifacts::semio::standards::v1::subsets::graph::schema::diff::{SemioGraphDiff, SemioGraphNodeList};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &ChangeNodeKind, base: &SemioGraphSnapshot) -> SemioGraphDiff {
    let mut nodes = base.nodes.clone();
    if let Some(node) = nodes.iter_mut().find(|n| n.id == payload.id) {
        node.kind = payload.new_kind.clone();
    }
    SemioGraphDiff { nodes: Some(SemioGraphNodeList { values: nodes }), edges: None }
}
//#endregion 🔖️Diff
