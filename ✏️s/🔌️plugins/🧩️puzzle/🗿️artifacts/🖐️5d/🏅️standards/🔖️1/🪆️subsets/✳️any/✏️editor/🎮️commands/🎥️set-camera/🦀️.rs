//! 🎥️ `set-camera` command.

use crate::editor::puzzle5d::config::{Puzzle5dCamera2d, Puzzle5dCamera3d};
use crate::editor::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::{to_dsl_value, Value};
use dsl::FromValue;

/// 📷️ The surface-agnostic setter. An explicit `surfaceId` wins; otherwise the ADDRESSED WINDOW
/// decides, because each pane persists only its own camera (`Puzzle5dBoardWindowConfig` carries
/// `camera2d`, `Puzzle5dWorldWindowConfig` carries `camera3d`) — guessing from the payload shape alone
/// wrote the flat camera for every world-pane pose that happened to omit `position`, and the world
/// pane's partition then dropped it silently.
pub fn set_camera(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(camera) = args.and_then(|value| value.get("camera")) else {
        return;
    };
    let flat = match args.and_then(|value| value.get("surfaceId")).and_then(|value| value.as_str()) {
        Some(board2d::SURFACE_ID) => true,
        Some(world3d::SURFACE_ID) => false,
        _ => ctx.window_kind == board2d::WINDOW_KIND_ID || camera.get("position").is_none(),
    };
    if flat {
        if let Ok(parsed) = Puzzle5dCamera2d::from_value(to_dsl_value(camera)) {
            ctx.scene.runtime.camera2d = parsed;
        }
    } else if let Ok(parsed) = Puzzle5dCamera3d::from_value(to_dsl_value(camera)) {
        ctx.scene.runtime.camera3d = parsed;
    }
}
