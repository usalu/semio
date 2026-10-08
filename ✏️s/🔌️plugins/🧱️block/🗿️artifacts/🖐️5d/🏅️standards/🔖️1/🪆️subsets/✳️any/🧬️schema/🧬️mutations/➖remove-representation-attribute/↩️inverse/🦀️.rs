//! ↩️ Inverse for `RemoveRepresentationAttribute`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveRepresentationAttribute, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    match existing.attributes.iter().enumerate().find(|(_, attribute)| attribute.key == payload.key) {
        Some((position, attribute)) => vec![super::super::add_representation_attribute::add_representation_attribute_at(payload.id.clone(), attribute.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
