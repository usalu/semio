//! ↩️ Inverse for `ChangeNodeKindIcon` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeKindIcon, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_icon::mutation::change_node_kind_icon(base.node_kind.icon.clone())]
}
//#endregion 🔖️Inverse
