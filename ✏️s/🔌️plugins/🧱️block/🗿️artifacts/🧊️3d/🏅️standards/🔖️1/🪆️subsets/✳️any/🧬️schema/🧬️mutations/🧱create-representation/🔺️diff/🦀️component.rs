//! 🔺️ Sparse diff builder for `CreateRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block3d::diff::Block3dDiff;
use crate::artifacts::block3d::diff::{Block3dRepresentationsDelta};
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateRepresentation, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { representations: Some(Block3dRepresentationsDelta { added: vec![payload.representation.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
