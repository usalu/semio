//! 🔺️ Diff for `MoveCamera2d`.

use semio_s_artifact_block_2d_model::Block2dSnapshot;
use semio_s_artifact_block_2d_diff::Block2dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::move_camera2d::MoveCamera2d, base: &Block2dSnapshot) -> protocol::MutationOutcome<Block2dDiff> {
    if !payload.new_x.is_finite() || !payload.new_y.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera position ({}, {}) is not finite.", payload.new_x, payload.new_y), ["camera2d"]);
    }
    if payload.new_x == base.camera2d.x && payload.new_y == base.camera2d.y {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Camera is already at ({}, {}).", payload.new_x, payload.new_y));
    }
    protocol::MutationOutcome::new(Block2dDiff { camera2d: Some(semio_s_plugin_block::BlockCamera2dPatch { x: Some(payload.new_x), y: Some(payload.new_y), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
