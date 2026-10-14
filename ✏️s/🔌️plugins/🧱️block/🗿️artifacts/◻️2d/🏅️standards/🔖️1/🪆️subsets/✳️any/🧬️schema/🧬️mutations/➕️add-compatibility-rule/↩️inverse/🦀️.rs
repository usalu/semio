//! ↩️ Inverse for `AddCompatibilityRule`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &crate::add_compatibility_rule::AddCompatibilityRule, base: &Block2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_compatibility_rule::RemoveCompatibilityRule> {
    Ok((|| {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return Vec::new();
    }
    vec![crate::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]

    })())
}
//#endregion 🔖️Inverse
