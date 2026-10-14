//! 🔺️ Diff for `CreateVortexKind`.

use semio_s_artifact_block_3d_model::Block3dSnapshot;
use semio_s_artifact_block_3d_diff::{Block3dDiff, Block3dVortexKindsDelta};

//#region 🔖️Diff
pub fn diff(payload: &crate::create_vortex_kind::CreateVortexKind, base: &Block3dSnapshot) -> protocol::MutationOutcome<Block3dDiff> {
    if semio_s_artifact_block_3d_model::vortex_kinds_of(base).iter().any(|item| item.id == payload.vortex_kind.id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("{} \"{}\" already exists", "vortex-kind", payload.vortex_kind.id), vec![payload.vortex_kind.id.clone()]);
    }
    protocol::MutationOutcome::new(Block3dDiff { vortex_kinds: Block3dVortexKindsDelta::insertion(semio_s_plugin_block::block_insert_index(semio_s_artifact_block_3d_model::vortex_kinds_of(base).len(), payload.index), payload.vortex_kind.clone()), ..Default::default() })
}
//#endregion 🔖️Diff
