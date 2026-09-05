//! 🔺️ Diff for `UpdatePart3d`.

use crate::artifacts::block5d::{Block5dPart3d, Block5dSnapshot};
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdatePart3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let part_3d = Block5dPart3d { orientation: payload.new_orientation, scale: payload.new_scale };
    if part_3d == base.part_3d {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "3D pose is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_3d: Some(part_3d), ..Default::default() })
}
//#endregion 🔖️Diff
