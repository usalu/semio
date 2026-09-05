//! ↩️ Inverse for `UpdatePart2d`.

use crate::artifacts::block5d::{Block5dPart2d, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdatePart2d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::update_part_2d::update_part_2d(base.part_2d.shape.clone(), base.part_2d.radius, base.part_2d.width, base.part_2d.height, base.part_2d.color.clone(), base.part_2d.icon_kind.clone())]
}
//#endregion 🔖️Inverse
