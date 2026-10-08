//! 🔺️ Diff for `UpdatePart2d`.

use crate::standards::v1::subsets::any::schema::diff::{Block5dDiff, Block5dPart2dPatch};
use crate::Block5dSnapshot;
use semio_s_plugin_block::{BlockOptionalNumber, BlockOptionalText};

//#region 🔖️Diff
pub fn diff(payload: &super::UpdatePart2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    let part_2d = &base.part_2d;
    if part_2d.shape == payload.new_shape
        && part_2d.radius == payload.new_radius
        && part_2d.width == payload.new_width
        && part_2d.height == payload.new_height
        && part_2d.color == payload.new_color
        && part_2d.icon_kind == payload.new_icon_kind
    {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "2D presentation is unchanged.");
    }
    let patch = Block5dPart2dPatch {
        shape: Some(BlockOptionalText { value: payload.new_shape.clone() }),
        radius: Some(BlockOptionalNumber { value: payload.new_radius }),
        width: Some(BlockOptionalNumber { value: payload.new_width }),
        height: Some(BlockOptionalNumber { value: payload.new_height }),
        color: Some(BlockOptionalText { value: payload.new_color.clone() }),
        icon_kind: Some(BlockOptionalText { value: payload.new_icon_kind.clone() }),
    };
    protocol::MutationOutcome::new(Block5dDiff { part_2d: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
