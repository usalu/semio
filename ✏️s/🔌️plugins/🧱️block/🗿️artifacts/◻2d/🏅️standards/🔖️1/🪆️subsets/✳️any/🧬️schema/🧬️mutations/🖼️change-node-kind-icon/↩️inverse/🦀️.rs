//! ↩️ Inverse for `ChangeNodeKindIcon`.

use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::ChangeNodeKindIcon, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_icon::change_node_kind_icon(base.node_kind.icon.clone())]
}
//#endregion 🔖️Inverse
