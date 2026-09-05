//! 🖼️ `set-camera-2d` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::{to_dsl_value, Value};
use dsl::FromValue;

pub fn set_camera_2d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = crate::editor::puzzle5d::config::Puzzle5dCamera2d::from_value(to_dsl_value(camera)) {
            ctx.scene.runtime.camera2d = parsed;
        }
    }
}
