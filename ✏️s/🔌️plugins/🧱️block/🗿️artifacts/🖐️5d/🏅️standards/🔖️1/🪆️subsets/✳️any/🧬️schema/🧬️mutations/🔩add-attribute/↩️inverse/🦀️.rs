//! ↩️ Inverse for `AddAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAttribute, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return Vec::new();
    }
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
