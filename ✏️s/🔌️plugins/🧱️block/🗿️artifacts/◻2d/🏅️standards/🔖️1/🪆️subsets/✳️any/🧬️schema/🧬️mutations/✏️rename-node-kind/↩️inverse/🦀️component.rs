//! ↩️ Inverse for `RenameNodeKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameNodeKind, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::rename_node_kind::mutation::rename_node_kind(base.node_kind.name.clone())]
}
//#endregion 🔖️Inverse
