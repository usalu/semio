//! ↩️ Inverse for `ChangeNodeKindVariant` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::ChangeNodeKindVariant, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::change_node_kind_variant::mutation::change_node_kind_variant(base.node_kind.variant.clone())]
}
//#endregion 🔖️Inverse
