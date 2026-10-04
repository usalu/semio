//! ↩️ Inverse for `RenameNode`.

use crate::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation;
use crate::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot;

//#region 🔖️Inverse
/// ↩️ The exact undo: one `rename-node` from the new id back to the old one (the edge cascade follows it back); nothing
/// when the rename would not apply to `base`.
pub fn inverse(payload: &super::RenameNode, base: &SemioGraphSnapshot) -> Vec<SemioGraphMutation> {
    let applies = !payload.new_id.value.is_empty() && payload.new_id != payload.id && base.nodes.iter().any(|node| node.id == payload.id) && !base.nodes.iter().any(|node| node.id == payload.new_id);
    applies.then(|| SemioGraphMutation::RenameNode(super::RenameNode { id: payload.new_id.clone(), new_id: payload.id.clone() })).into_iter().collect()
}
//#endregion 🔖️Inverse
