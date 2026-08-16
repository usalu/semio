//! 🎥️ `set-projection` command.

use crate::editor::puzzle3d::config::puzzle3d_camera_distance;
use crate::editor::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use serde_json::Value;

/// 🧭️ `setProjection`/`setProjectionParam` share one arm — a projection change that also moves the
/// camera pose re-derives position/up from the new orientation around the unchanged target.
pub fn set_projection(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let moves_pose = world3d_projection_action_moves_pose(action, args);
    apply_world3d_projection_action(&mut ctx.scene.runtime.camera.projection, action, args);
    if moves_pose {
        let distance = puzzle3d_camera_distance(&ctx.scene.runtime.camera);
        let (position, up) = world3d_projection_pose(&ctx.scene.runtime.camera.projection, ctx.scene.runtime.camera.target, distance);
        ctx.scene.runtime.camera.position = position;
        ctx.scene.runtime.camera.up = Some(up);
    }
}
