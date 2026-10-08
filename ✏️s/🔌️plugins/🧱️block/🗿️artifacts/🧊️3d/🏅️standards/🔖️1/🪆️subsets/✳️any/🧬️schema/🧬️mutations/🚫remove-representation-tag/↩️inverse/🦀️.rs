//! ↩️ Inverse for `RemoveRepresentationTag`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RemoveRepresentationTag, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if let Some(position) = existing.tags.iter().position(|tag| *tag == payload.tag) {
        vec![super::super::add_representation_tag::add_representation_tag_at(payload.id.clone(), payload.tag.clone(), position as u32)]
    } else {
        Vec::new()
    }

    })())
}
//#endregion 🔖️Inverse
