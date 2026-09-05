//! 🔺️ Diff for `MoveCamera2d`.

use crate::BlockCamera2d;
use crate::artifacts::block2d::Block2dSnapshot;
use crate::artifacts::block2d::diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &super::MoveCamera2d, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera position ({}, {}) is not finite.", payload.new_x, payload.new_y), ["camera2d"]);
    }
    if payload.new_x == base.camera2d.x && payload.new_y == base.camera2d.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Camera is already at ({}, {}).", payload.new_x, payload.new_y));
    }
    protocol::MutationOutcome::new(Block2dDiff { camera2d: Some(BlockCamera2d { x: payload.new_x, y: payload.new_y, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
