//! 🔺️ Sparse diff builder for `UpdatePresentation` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block2d::diff::Block2dDiff;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::{Block2dPresentation};

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::UpdatePresentation, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    protocol::MutationOutcome::new(Block2dDiff { presentation: Some(Block2dPresentation { shape: payload.new_shape.clone(), radius: payload.new_radius, width: payload.new_width, height: payload.new_height, color: payload.new_color.clone(), icon_kind: payload.new_icon_kind.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
