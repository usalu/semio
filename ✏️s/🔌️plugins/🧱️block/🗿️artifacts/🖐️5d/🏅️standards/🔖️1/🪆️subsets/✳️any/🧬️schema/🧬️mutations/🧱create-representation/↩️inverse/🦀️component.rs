//! ↩️ Inverse for `CreateRepresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateRepresentation, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_representation::mutation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
