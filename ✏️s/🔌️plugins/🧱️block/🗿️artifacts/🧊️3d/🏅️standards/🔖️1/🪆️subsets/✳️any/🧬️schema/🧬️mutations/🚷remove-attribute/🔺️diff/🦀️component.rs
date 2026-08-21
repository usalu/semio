//! 🔺️ Sparse diff builder for `RemoveAttribute` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dAttributesDelta;
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::RemoveAttribute, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.attributes.iter().any(|item| item.key == payload.key) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "attribute", payload.key), vec![payload.key.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { attributes: Some(Block3dAttributesDelta { removed: vec![payload.key.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
