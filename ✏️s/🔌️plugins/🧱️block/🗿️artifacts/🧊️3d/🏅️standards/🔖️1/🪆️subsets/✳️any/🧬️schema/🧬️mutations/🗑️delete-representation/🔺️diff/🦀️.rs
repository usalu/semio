//! 🔺️ Diff for `DeleteRepresentation`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::Block3dDiff;
use semio_s_plugin_block::{BlockRepresentationsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_representation::DeleteRepresentation, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.representations.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "representation", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { representations: BlockRepresentationsDelta::removal(&base.representations, base.representations.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
