//! ↩️ Inverse for `RemoveRepresentationTag`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::remove_representation_tag::RemoveRepresentationTag, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_representation_tag::AddRepresentationTag> {
    Ok((|| {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    if let Some(position) = existing.tags.iter().position(|tag| *tag == payload.tag) {
        vec![crate::add_representation_tag::add_representation_tag_at(payload.id.clone(), payload.tag.clone(), position as u32)]
    } else {
        Vec::new()
    }

    })())
}
//#endregion 🔖️Inverse
