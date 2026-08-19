//! 🔺️ Sparse diff builder for `MoveCamera2d` — real handcrafted delta, never apply-then-capture.
use crate::artifacts::block5d::diff::Block5dDiff;
use crate::artifacts::block5d::Block5dSnapshot;
use crate::{BlockCamera2d};

//#region 🔖️Diff
pub async fn diff(payload: &super::mutation::MoveCamera2d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera position ({}, {}) is not finite.", payload.new_x, payload.new_y), ["camera2d"]);
    }
    if payload.new_x == base.camera2d.x && payload.new_y == base.camera2d.y {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", format!("Camera is already at ({}, {}).", payload.new_x, payload.new_y));
    }
    protocol::MutationOutcome::new(Block5dDiff { camera2d: Some(BlockCamera2d { x: payload.new_x, y: payload.new_y, ..base.camera2d.clone() }), ..Default::default() })
}
//#endregion 🔖️Diff
