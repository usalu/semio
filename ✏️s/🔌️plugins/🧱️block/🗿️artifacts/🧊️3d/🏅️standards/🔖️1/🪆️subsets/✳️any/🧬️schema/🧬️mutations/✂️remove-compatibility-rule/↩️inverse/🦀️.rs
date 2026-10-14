//! ↩️ Inverse for `RemoveCompatibilityRule`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::remove_compatibility_rule::RemoveCompatibilityRule, base: &Block3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::add_compatibility_rule::AddCompatibilityRule> {
    Ok((|| {
    match base.compatibility.iter().enumerate().find(|(_, item)| item.id == payload.id) {
        Some((position, existing)) => vec![crate::add_compatibility_rule::add_compatibility_rule_at(existing.clone(), position as u32)],
        None => Vec::new(),
    }

    })())
}
//#endregion 🔖️Inverse
