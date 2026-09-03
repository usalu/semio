//! ↩️ Inverse for `ChangeNodeKindDescription`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeNodeKindDescription, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_description::change_node_kind_description(base.node_kind.description.clone())]
}
//#endregion 🔖️Inverse
