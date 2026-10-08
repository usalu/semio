//! ↩️ Inverse for `AddAttribute`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddAttribute, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return Vec::new();
    }
    vec![super::super::remove_attribute::remove_attribute(payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
