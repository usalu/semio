//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::Block2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddCompatibilityRule, base: &Block2dSnapshot) -> Result<Vec<Block2dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return Vec::new();
    }
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]

    })())
}
//#endregion 🔖️Inverse
