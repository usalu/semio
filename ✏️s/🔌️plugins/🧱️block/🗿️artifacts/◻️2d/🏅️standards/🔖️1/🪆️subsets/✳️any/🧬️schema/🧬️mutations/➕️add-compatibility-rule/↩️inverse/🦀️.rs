//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::Block2dSnapshot;
use crate::mutations::Block2dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddCompatibilityRule, _base: &Block2dSnapshot) -> Vec<Block2dMutation> {
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
