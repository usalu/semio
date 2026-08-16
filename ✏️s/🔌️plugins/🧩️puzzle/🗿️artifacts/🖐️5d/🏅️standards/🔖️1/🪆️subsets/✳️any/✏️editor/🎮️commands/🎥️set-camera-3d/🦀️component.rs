//! 🎥️ `set-camera-3d` command.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::{gumball_target_world, Puzzle5dActionCtx};
use serde_json::Value;

pub fn set_camera_3d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera3d = parsed;
        }
    }
}
