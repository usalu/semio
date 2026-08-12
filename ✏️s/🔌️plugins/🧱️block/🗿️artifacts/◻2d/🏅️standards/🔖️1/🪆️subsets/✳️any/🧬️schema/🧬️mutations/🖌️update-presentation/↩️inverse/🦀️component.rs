//! ↩️ Inverse for `UpdatePresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block2d::mutations::Block2dMutation;
use crate::artifacts::block2d::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::UpdatePresentation, base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::update_presentation::mutation::update_presentation(base.presentation.shape.clone(), base.presentation.radius, base.presentation.width, base.presentation.height, base.presentation.color.clone(), base.presentation.icon_kind.clone())]
}
//#endregion 🔖️Inverse
