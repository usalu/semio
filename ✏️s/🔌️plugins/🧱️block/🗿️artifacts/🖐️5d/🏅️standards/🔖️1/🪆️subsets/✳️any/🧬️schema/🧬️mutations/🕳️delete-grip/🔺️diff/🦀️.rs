//! 🔺️ Diff for `DeleteGrip`.

use crate::artifacts::block5d::Block5dSnapshot;
use crate::artifacts::block5d::diff::{Block5dDiff, Block5dGripsDelta};

//#region 🔖️Diff
pub fn diff(payload: &super::DeleteGrip, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.grips.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grips: Some(Block5dGripsDelta { removed: vec![payload.id.clone()], ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
