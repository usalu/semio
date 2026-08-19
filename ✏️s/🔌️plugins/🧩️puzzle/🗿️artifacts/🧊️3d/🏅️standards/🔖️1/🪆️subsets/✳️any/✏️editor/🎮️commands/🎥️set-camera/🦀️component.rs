//! 🎥️ `set-camera` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_camera(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = serde_json::from_value(camera.clone()) {
            ctx.scene.runtime.camera = parsed;
        }
    }
}
