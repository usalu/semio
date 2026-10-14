//! 🔺️ Diff for `DeleteVortex`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_vortex::DeleteVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !base.vortices.iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Block3dVorticesDelta::removal(&base.vortices, base.vortices.iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
