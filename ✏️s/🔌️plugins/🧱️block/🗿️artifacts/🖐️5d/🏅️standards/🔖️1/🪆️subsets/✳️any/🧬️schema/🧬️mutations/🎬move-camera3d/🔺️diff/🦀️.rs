//! 🔺️ Diff for `MoveCamera3d`.

use semio_s_artifact_block_5d_model::Block5dSnapshot;
use semio_s_artifact_block_5d_diff::Block5dDiff;

//#region 🔖️Diff
pub fn diff(payload: &crate::move_camera3d::MoveCamera3d, base: &Block5dSnapshot) -> protocol::MutationOutcome<Block5dDiff> {
    if payload.new_position.iter().any(|c| !c.is_finite()) || payload.new_target.iter().any(|c| !c.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("Camera position {:?} / target {:?} is not finite.", payload.new_position, payload.new_target), ["camera3d"]);
    }
    if payload.new_position == base.camera3d.position && payload.new_target == base.camera3d.target {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Camera is already at {:?}.", payload.new_position));
    }
    protocol::MutationOutcome::new(Block5dDiff { camera3d: Some(semio_s_plugin_block::BlockCamera3dPatch { position: Some(payload.new_position), target: Some(payload.new_target), ..Default::default() }), ..Default::default() })
}
//#endregion 🔖️Diff
