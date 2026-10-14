//! ↩️ Inverse for `AddRepresentationAttribute`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::add_representation_attribute::AddRepresentationAttribute, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_representation_attribute::RemoveRepresentationAttribute> {
    Ok((|| {
    if base.representations.iter().any(|item| item.id == payload.id && item.attributes.iter().any(|attribute| attribute.key == payload.attribute.key)) {
        return Vec::new();
    }
    vec![crate::remove_representation_attribute::remove_representation_attribute(payload.id.clone(), payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
