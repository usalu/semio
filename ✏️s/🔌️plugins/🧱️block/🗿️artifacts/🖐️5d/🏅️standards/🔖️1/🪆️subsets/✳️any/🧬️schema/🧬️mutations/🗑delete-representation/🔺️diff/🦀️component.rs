//! 🔺️ Sparse diff builder for `DeleteRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::DeleteRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.representations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
