//! ↩️ Inverse for `RemoveRepresentationAttribute`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::remove_representation_attribute::RemoveRepresentationAttribute, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_representation_attribute::AddRepresentationAttribute> {
    Ok((|| {
    let Some(existing) = base.representations.iter().find(|item| item.id == payload.id) else {
        return Vec::new();
    };
    match existing.attributes.iter().enumerate().find(|(_, attribute)| attribute.key == payload.key) {
        Some((position, attribute)) => vec![crate::add_representation_attribute::add_representation_attribute_at(payload.id.clone(), attribute.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
