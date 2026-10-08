//! 🔺️ Diff for `UpdatePart3d`.

use crate::standards::v1::subsets::any::schema::diff::{Block5dDiff, Block5dPart3dPatch};
use crate::Block5dSnapshot;
use semio_s_plugin_block::{BlockOptionalOrientation, BlockOptionalScale};

//#region 🔖️Diff
pub fn diff(payload: &super::UpdatePart3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if base.part_3d.orientation == payload.new_orientation && base.part_3d.scale == payload.new_scale {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "3D pose is unchanged.");
    }
    let patch = Block5dPart3dPatch { orientation: Some(BlockOptionalOrientation { value: payload.new_orientation }), scale: Some(BlockOptionalScale { value: payload.new_scale }) };
    protocol::MutationOutcome::new(Block5dDiff { part_3d: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
