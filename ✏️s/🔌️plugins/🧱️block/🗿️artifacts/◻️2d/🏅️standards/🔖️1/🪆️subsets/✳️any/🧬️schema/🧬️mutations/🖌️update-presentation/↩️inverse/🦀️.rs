//! ↩️ Inverse for `UpdatePresentation`.

use crate::artifacts::block2d::{Block2dPresentation, Block2dSnapshot};
use crate::artifacts::block2d::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::UpdatePresentation, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::update_presentation::update_presentation(
        base.presentation.shape.clone(),
        base.presentation.radius,
        base.presentation.width,
        base.presentation.height,
        base.presentation.color.clone(),
        base.presentation.icon_kind.clone(),
    )]
}
//#endregion 🔖️Inverse
