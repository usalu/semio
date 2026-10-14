//! 🔺️ Diff for `DeleteVortexKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::{Block3dDiff, Block3dVortexKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::delete_vortex_kind::DeleteVortexKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if !semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().any(|item| item.id == payload.id) {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("{} \"{}\" not found", "vortex-kind", payload.id), vec![payload.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortex_kinds: Block3dVortexKindsDelta::removal(&semio_s_artifact_block_3d_model::vortex_kinds_of(base), semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().position(|item| item.id == payload.id).unwrap_or(usize::MAX)), ..Default::default() })
}
//#endregion 🔖️Diff
