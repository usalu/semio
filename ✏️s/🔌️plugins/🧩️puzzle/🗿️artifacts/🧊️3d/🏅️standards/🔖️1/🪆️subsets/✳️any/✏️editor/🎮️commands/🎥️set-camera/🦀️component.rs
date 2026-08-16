//! 🎥️ `set-camera` command.

use crate::editor::puzzle3d::config::puzzle3d_camera_distance;
use crate::editor::puzzle3d::{apply_puzzle3d_focus_selection, Puzzle3dActionCtx};
use semio_framework_plugin::{apply_world3d_projection_action, world3d_projection_action_moves_pose, world3d_projection_pose};
use serde_json::Value;

pub fn set_camera(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera = parsed;
        }
    }
}
