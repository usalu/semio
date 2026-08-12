//! ↩️ Inverse for `CreateGrip` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateGrip, base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip::mutation::delete_grip(payload.grip.id.clone())]
}
//#endregion 🔖️Inverse
