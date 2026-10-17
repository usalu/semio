//! ↩️ Inverse for `ResizeNode`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `resize-node` back to the node's BASE size; nothing when the node is absent or keeps its size.
pub fn inverse<Op>(payload: &super::ResizeNode, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::resize_node::ResizeNode>,
{
    base.nodes
        .iter()
        .find(|node| node.id == payload.id && (node.width, node.height) != (payload.width, payload.height))
        .map(|node| Op::from(super::ResizeNode { id: payload.id.clone(), width: node.width, height: node.height }))
        .into_iter()
        .collect()
}
//#endregion 🔖️Inverse
