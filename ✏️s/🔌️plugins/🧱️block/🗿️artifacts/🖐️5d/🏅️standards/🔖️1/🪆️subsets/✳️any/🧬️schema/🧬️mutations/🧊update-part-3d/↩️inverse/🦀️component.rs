//! ↩️ Inverse for `UpdatePart3d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::UpdatePart3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::update_part_3d::mutation::update_part_3d(base.part_3d.orientation, base.part_3d.scale)]
}
//#endregion 🔖️Inverse
