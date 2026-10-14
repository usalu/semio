//! 🔺️ Diff for `CreateVortex`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::{Block3dDiff, Block3dVorticesDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_vortex::CreateVortex, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if base.vortices.iter().any(|item| item.id == payload.vortex.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "vortex", payload.vortex.id), vec![payload.vortex.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortices: Block3dVorticesDelta::insertion(semio_s_plugin_block::block_insert_index(base.vortices.len(), payload.index), payload.vortex.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
