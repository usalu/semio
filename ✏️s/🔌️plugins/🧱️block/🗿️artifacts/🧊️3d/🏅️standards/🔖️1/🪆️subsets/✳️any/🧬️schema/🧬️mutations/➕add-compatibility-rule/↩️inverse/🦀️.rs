//! ↩️ Inverse for `AddCompatibilityRule`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::AddCompatibilityRule, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::remove_compatibility_rule::remove_compatibility_rule(payload.rule.id.clone())]
}
//#endregion 🔖️Inverse
