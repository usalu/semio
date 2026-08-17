//! 🎥️ `set-camera-2d` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

pub fn set_camera_2d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera2d = parsed;
        }
    }
}
