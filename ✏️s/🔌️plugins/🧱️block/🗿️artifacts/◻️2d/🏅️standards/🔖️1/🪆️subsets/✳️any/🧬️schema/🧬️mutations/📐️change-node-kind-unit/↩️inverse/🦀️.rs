//! ↩️ Inverse for `ChangeNodeKindUnit`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeNodeKindUnit, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_unit::change_node_kind_unit(base.node_kind.unit.clone())]
}
//#endregion 🔖️Inverse
