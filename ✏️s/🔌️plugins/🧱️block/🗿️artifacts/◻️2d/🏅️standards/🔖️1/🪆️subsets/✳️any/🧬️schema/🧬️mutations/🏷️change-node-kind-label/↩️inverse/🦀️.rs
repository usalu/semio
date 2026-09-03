//! ↩️ Inverse for `ChangeNodeKindLabel`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeNodeKindLabel, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_label::change_node_kind_label(base.node_kind.label.clone())]
}
//#endregion 🔖️Inverse
