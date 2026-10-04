//! 🎦️ `set-camera-3d` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use semio_framework_pack_json::{to_dsl_value, Value};
use semio_framework_value::FromValue;

pub fn set_camera_3d(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = crate::editor::puzzle5d::config::Puzzle5dCamera3d::from_value(to_dsl_value(camera)) {
            ctx.scene.runtime.camera3d = parsed;
        }
    }
}
