//! ↩️ Inverse for `CreateRepresentation`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateRepresentation, _base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::delete_representation::delete_representation(payload.representation.id.clone())]

    })())
}
//#endregion 🔖️Inverse
