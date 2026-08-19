//! ↩️ Inverse for `UpdatePart2d` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::mutation::UpdatePart2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::update_part_2d::mutation::update_part_2d(base.part_2d.shape.clone(), base.part_2d.radius, base.part_2d.width, base.part_2d.height, base.part_2d.color.clone(), base.part_2d.icon_kind.clone())]
}
//#endregion 🔖️Inverse
