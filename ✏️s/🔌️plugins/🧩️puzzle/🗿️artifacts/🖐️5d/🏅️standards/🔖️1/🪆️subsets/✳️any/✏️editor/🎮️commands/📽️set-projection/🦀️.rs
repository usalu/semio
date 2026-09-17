//! 🎥️ `set-projection` command — `setProjection`/`setProjectionParam` share one arm: a projection
//! change that also moves the camera pose re-derives position/up from the new orientation around the
//! unchanged target, a pure parameter tweak leaves the pose alone.

use crate::editor::puzzle5d::config::puzzle5d_camera3d_distance;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};

pub fn set_projection(ctx: &mut Puzzle5dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let moves_pose = world3d_projection_action_moves_pose(action, args);
    apply_world3d_projection_action(&mut ctx.scene.runtime.camera3d.projection, action, args);
    if moves_pose {
        let distance = puzzle5d_camera3d_distance(&ctx.scene.runtime.camera3d);
        let (position, up) = world3d_projection_pose(&ctx.scene.runtime.camera3d.projection, ctx.scene.runtime.camera3d.target, distance);
        ctx.scene.runtime.camera3d.position = position;
        ctx.scene.runtime.camera3d.up = Some(up);
    }
}
