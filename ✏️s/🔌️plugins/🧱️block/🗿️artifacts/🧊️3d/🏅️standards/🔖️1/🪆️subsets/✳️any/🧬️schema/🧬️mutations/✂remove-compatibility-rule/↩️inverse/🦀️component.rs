//! ↩️ Inverse for `RemoveCompatibilityRule` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::RemoveCompatibilityRule, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.compatibility.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::add_compatibility_rule::mutation::add_compatibility_rule(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
