//! ↩️ Inverse for `AddRepresentationAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationAttribute, _base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![super::super::remove_representation_attribute::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
