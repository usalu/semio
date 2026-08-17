//! ↩️ Inverse for `ChangeNodeKindUnit` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::mutation::ChangeNodeKindUnit, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_unit::mutation::change_node_kind_unit(base.node_kind.unit.clone())]
}
//#endregion 🔖️Inverse
