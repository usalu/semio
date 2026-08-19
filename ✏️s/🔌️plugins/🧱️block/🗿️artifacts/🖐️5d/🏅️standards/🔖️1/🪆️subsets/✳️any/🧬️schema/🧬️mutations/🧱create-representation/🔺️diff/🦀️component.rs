//! 🔺️ Sparse diff builder for `CreateRepresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::diff::{Block5dRepresentationsDelta};
use crate::artifacts::block5d::Block5dSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::CreateRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { added: vec![payload.representation.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
