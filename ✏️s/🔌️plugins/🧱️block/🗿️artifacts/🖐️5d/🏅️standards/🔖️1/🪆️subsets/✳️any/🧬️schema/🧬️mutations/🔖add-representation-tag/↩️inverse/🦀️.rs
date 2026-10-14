//! ↩️ Inverse for `AddRepresentationTag`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::add_representation_tag::AddRepresentationTag, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_representation_tag::RemoveRepresentationTag> {
    Ok((|| {
    if base.representations.iter().any(|item| item.id == payload.id && item.tags.contains(&payload.tag)) {
        return Vec::new();
    }
    vec![crate::remove_representation_tag::remove_representation_tag(payload.id.clone(), payload.tag.clone())]

    })())
}
//#endregion 🔖️Inverse
