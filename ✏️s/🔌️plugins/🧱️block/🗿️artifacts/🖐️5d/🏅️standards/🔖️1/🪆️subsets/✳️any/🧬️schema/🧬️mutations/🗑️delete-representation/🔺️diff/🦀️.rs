//! 🔺️ Diff for `DeleteRepresentation`.

use crate::Block5dSnapshot;
use crate::diff::{Block5dDiff, Block5dRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.representations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: Some(Block5dRepresentationsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
