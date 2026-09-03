//! 🎥️ `set-camera` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_camera(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(camera)) {
            ctx.scene.runtime.camera = parsed;
        }
    }
}
