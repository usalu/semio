//! 🔺️ Sparse diff builder for `DeleteRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::Block3dRepresentationsDelta;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteRepresentation, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.representations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { representations: Some(Block3dRepresentationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
