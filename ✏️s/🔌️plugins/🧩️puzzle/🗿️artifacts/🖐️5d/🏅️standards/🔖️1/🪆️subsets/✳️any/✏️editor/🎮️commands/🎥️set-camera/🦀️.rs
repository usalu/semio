//! 🎥️ `set-camera` command.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::{to_dsl_value, Value};
use dsl::FromValue;

/// 📷️ The surface-agnostic setter: the flat camera wins when the surface is the board (or the payload
/// carries no `position`), otherwise the volume camera.
pub fn set_camera(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(camera) = args.and_then(|value| value.get("camera")) else {
        return;
    };
    let surface_id = args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()).unwrap_or("");
    if surface_id == board2d::SURFACE_ID || camera.get("position").is_none() {
        if let Ok(parsed) = Puzzle5dCamera2d::from_value(to_dsl_value(camera)) {
            ctx.scene.runtime.camera2d = parsed;
        }
    } else if let Ok(parsed) = Puzzle5dCamera3d::from_value(to_dsl_value(camera)) {
        ctx.scene.runtime.camera3d = parsed;
    }
}
