//! ↩️ Inverse for `RenameNode`.

use crate::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `rename-node` from the new id back to the old one (the edge cascade follows it back); nothing
/// when the rename would not apply to `base`.
pub fn inverse<Op>(payload: &super::RenameNode, base: &SemioGraphSnapshot) -> Vec<Op>
where
    Op: From<crate::schema::mutations::rename_node::RenameNode>,
{
    let applies = !payload.new_id.value.is_empty() && payload.new_id != payload.id && base.nodes.iter().any(|node| node.id == payload.id) && !base.nodes.iter().any(|node| node.id == payload.new_id);
    applies.then(|| Op::from(super::RenameNode { id: payload.new_id.clone(), new_id: payload.id.clone() })).into_iter().collect()
}
//#endregion 🔖️Inverse
