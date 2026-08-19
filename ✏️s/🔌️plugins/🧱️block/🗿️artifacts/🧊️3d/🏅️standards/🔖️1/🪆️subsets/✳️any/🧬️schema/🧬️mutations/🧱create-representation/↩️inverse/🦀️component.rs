//! ↩️ Inverse for `CreateRepresentation` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateRepresentation, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_representation::mutation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
