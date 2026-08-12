//! ↩️ Inverse for `ChangeNodeKindLabel` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeNodeKindLabel, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_label::mutation::change_node_kind_label(base.node_kind.label.clone())]
}
//#endregion 🔖️Inverse
