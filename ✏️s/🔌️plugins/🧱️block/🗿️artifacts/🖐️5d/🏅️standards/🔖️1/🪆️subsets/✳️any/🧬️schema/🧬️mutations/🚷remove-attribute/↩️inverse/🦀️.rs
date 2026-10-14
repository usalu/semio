//! ↩️ Inverse for `RemoveAttribute`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::remove_attribute::RemoveAttribute, base: &Block5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_attribute::AddAttribute> {
    Ok((|| {
    match base.attributes.iter().enumerate().find(|(_, item)| item.key == payload.key) {
        Some((position, existing)) => vec![crate::add_attribute::add_attribute_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
