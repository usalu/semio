//! ↩️ Inverse for `ChangeNodeKindDescription` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeNodeKindDescription, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_description::mutation::change_node_kind_description(base.node_kind.description.clone())]
}
//#endregion 🔖️Inverse
