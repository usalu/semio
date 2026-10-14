//! ↩️ Inverse for `AddAttribute`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::add_attribute::AddAttribute, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_attribute::RemoveAttribute> {
    Ok((|| {
    if base.attributes.iter().any(|item| item.key == payload.attribute.key) {
        return Vec::new();
    }
    vec![crate::remove_attribute::remove_attribute(payload.attribute.key.clone())]

    })())
}
//#endregion 🔖️Inverse
