//! ↩️ Inverse for `AddRepresentationTag`.

use crate::Block5dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddRepresentationTag, base: &Block5dSnapshot) -> Result<Vec<Block5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.representations.iter().any(|item| item.id == payload.id && item.tags.contains(&payload.tag)) {
        return Vec::new();
    }
    vec![super::super::remove_representation_tag::remove_representation_tag(payload.id.clone(), payload.tag.clone())]

    })())
}
//#endregion 🔖️Inverse
