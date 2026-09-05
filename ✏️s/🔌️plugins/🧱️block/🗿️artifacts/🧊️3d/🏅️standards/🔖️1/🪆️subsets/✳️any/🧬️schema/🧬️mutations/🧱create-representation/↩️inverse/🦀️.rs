//! ↩️ Inverse for `CreateRepresentation`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateRepresentation, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_representation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
