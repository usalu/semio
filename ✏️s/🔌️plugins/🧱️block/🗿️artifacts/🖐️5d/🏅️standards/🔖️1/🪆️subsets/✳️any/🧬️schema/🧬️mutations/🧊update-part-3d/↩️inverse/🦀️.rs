//! ↩️ Inverse for `UpdatePart3d`.

use crate::artifacts::block5d::{Block5dPart3d, Block5dSnapshot};
use crate::artifacts::block5d::mutations::Block5dMutation;

//#region 🔖️Inverse
pub async fn inverse(_payload: &super::UpdatePart3d, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::update_part_3d::mutation::update_part_3d(base.part_3d.orientation, base.part_3d.scale)]
}
//#endregion 🔖️Inverse
