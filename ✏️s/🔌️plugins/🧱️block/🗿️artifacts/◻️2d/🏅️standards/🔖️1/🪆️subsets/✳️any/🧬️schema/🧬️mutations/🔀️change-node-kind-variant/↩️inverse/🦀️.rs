//! ↩️ Inverse for `ChangeNodeKindVariant`.

use crate::Block2dSnapshot;
use crate::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeNodeKindVariant, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_variant::change_node_kind_variant(base.node_kind.variant.clone())]
}
//#endregion 🔖️Inverse
