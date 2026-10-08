//! ↩️ Inverse for `AddRepresentationAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationAttribute, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.representations.iter().any(|item| item.id == payload.id && item.attributes.iter().any(|attribute| attribute.key == payload.attribute.key)) {
        return Vec::new();
    }
    vec![super::super::remove_representation_attribute::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
