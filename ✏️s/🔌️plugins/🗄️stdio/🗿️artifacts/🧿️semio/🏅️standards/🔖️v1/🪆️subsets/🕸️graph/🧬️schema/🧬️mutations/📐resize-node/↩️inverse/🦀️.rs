//! ↩️ Inverse for `ResizeNode`.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `resize-node` back to the node's BASE size; nothing when the node is absent or keeps its size.
pub fn inverse(payload: &super::ResizeNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    base.nodes
        .iter()
        .find(|node| node.id == payload.id && (node.width, node.height) != (payload.width, payload.height))
        .map(|node| SemioGraphMutation::ResizeNode(super::ResizeNode { id: payload.id.clone(), width: node.width, height: node.height }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
