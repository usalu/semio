//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddCompatibilityRule, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
