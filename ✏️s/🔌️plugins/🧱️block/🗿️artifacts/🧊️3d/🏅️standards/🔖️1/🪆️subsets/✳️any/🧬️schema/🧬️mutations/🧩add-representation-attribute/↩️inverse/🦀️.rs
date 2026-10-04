//! ↩️ Inverse for `AddRepresentationAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationAttribute, _base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::remove_representation_attribute::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
