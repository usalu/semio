//! 🔺️ Diff for `CreateRepresentation`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::CreateRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.representations.iter().any(|item| item.id == payload.representation.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "representation", payload.representation.id), vec![payload.representation.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { added: vec![payload.representation.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
