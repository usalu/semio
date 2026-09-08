//! ↩️ Inverse for `CreateRepresentation`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateRepresentation, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_representation::delete_representation(payload.representation.id.clone())]
}
//#endregion 🔖️Inverse
