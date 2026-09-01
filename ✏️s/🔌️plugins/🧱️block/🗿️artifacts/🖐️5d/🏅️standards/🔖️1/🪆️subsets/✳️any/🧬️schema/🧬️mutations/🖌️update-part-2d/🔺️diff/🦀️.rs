//! 🔺️ Diff for `UpdatePart2d`.

use crate::artifacts::block5d::{Block5dPart2d, Block5dSnapshot};
use crate::artifacts::block5d::diff::Block5dDiff;

//#region 🔖️Diff
pub async fn diff(payload: &super::UpdatePart2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let part_2d = Block5dPart2d { shape: payload.new_shape.clone(), radius: payload.new_radius, width: payload.new_width, height: payload.new_height, color: payload.new_color.clone(), icon_kind: payload.new_icon_kind.clone() };
    if part_2d == base.part_2d {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "2D presentation is unchanged.");
    }
    protocol::MutationOutcome::new(Block5dDiff { part_2d: Some(part_2d), ..Default::default() })
}
//#endregion 🔖️Diff
