//! 🔺️ Diff for `UpdatePresentation`.

use crate::standards::v1::subsets::any::schema::diff::{Block2dDiff, Block2dPresentationPatch};
use crate::Block2dSnapshot;
use semio_s_plugin_block::{BlockOptionalNumber, BlockOptionalText};

//#region 🔖️Diff
pub fn diff(payload: &super::UpdatePresentation, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let presentation = &base.presentation;
    if presentation.shape == payload.new_shape
        && presentation.radius == payload.new_radius
        && presentation.width == payload.new_width
        && presentation.height == payload.new_height
        && presentation.color == payload.new_color
        && presentation.icon_kind == payload.new_icon_kind
    {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Presentation is unchanged.");
    }
    let patch = Block2dPresentationPatch {
        shape: Some(BlockOptionalText { value: payload.new_shape.clone() }),
        radius: Some(BlockOptionalNumber { value: payload.new_radius }),
        width: Some(BlockOptionalNumber { value: payload.new_width }),
        height: Some(BlockOptionalNumber { value: payload.new_height }),
        color: Some(BlockOptionalText { value: payload.new_color.clone() }),
        icon_kind: Some(BlockOptionalText { value: payload.new_icon_kind.clone() }),
    };
    protocol::MutationOutcome::new(Block2dDiff { presentation: Some(patch), ..Default::default() })
}
//#endregion 🔖️Diff
