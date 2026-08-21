//! 🔺️ Sparse diff builder for `RemoveCompatibilityRule` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dCompatibilityDelta;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveCompatibilityRule, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.compatibility.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "compatibility-rule", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { compatibility: Some(Block3dCompatibilityDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
