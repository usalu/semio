//! 🔺️ Diff for `DeleteRepresentation`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_representation::DeleteRepresentation, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !base.representations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block5dDiff { representations: BlockRepresentationsDelta::removal(&base.representations, base.representations.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
