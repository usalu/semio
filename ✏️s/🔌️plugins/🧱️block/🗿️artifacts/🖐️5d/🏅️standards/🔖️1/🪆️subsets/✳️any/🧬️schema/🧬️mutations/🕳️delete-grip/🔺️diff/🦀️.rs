//! 🔺️ Diff for `DeleteGrip`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::{Block5dDiff, Block5dGripsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_grip::DeleteGrip, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.grips.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "grip", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { grips: Block5dGripsDelta::removal(&base.grips, base.grips.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
