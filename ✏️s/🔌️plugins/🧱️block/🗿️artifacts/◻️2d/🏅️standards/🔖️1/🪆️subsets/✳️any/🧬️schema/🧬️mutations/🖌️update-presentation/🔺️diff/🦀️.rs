//! 🔺️ Diff for `UpdatePresentation`.

use semio_s_artifact_block_2d_diff::{Block2dDiff, Block2dPresentationPatch};
use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_plugin_block::{BlockOptionalNumber, BlockOptionalText};

//#region 🔖️Diff
pub fn diff(payload: &crate::update_presentation::UpdatePresentation, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
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
