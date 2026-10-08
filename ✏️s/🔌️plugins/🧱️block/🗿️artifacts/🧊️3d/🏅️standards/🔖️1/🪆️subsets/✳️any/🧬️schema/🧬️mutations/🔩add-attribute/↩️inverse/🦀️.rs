//! ↩️ Inverse for `AddAttribute`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAttribute, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return Vec::new();
    }
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
