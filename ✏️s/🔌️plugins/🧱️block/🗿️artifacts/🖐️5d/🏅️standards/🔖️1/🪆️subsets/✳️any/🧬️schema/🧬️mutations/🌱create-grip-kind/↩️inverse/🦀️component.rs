//! ↩️ Inverse for `CreateGripKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block5d::mutations::Block5dMutation;
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateGripKind, _base: &Block5dSnapshot) -> Vec<Block5dMutation> {
    vec![super::super::delete_grip_kind::mutation::delete_grip_kind(payload.grip_kind.id.clone())]
}
//#endregion 🔖️Inverse
