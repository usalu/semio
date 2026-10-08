//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::Block3dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddCompatibilityRule, base: &Block3dSnapshot) -> Result<Vec<Block3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    if base.compatibility.iter().any(|item| item.id == payload.rule.id) {
        return Vec::new();
    }
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]

    })())
}
//#endregion 🔖️Inverse
