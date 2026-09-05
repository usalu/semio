//! 🔺️ Diff for `UpdatePresentation`.

use crate::artifacts::block2d::{Block2dPresentation, Block2dSnapshot};
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdatePresentation, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    let presentation = Block2dPresentation { shape: payload.new_shape.clone(), radius: payload.new_radius, width: payload.new_width, height: payload.new_height, color: payload.new_color.clone(), icon_kind: payload.new_icon_kind.clone() };
    if presentation == base.presentation {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Presentation is unchanged.");
    }
    protocol::MutationOutcome::new(Block2dDiff { presentation: Some(presentation), ..Default::default() })
}
//#endregion 🔖️Diff
