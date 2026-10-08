//! ↩️ Inverse for `RemoveAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveAttribute, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.attributes.iter().enumerate().find(|(_, item)| item.key == payload.key) {
        Some((position, existing)) => vec![super::super::add_attribute::add_attribute_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
