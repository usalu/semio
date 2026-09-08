//! ↩️ Inverse for `ChangeNodeKindLabel`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNodeKindLabel, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_label::change_node_kind_label(base.node_kind.label.clone())]
}
//#endregion 🔖️Inverse
