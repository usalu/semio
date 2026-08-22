//! 🎥️ `set-camera` command.

use crate::editor::puzzle2d::{puzzle2d_window_only_scope, set_runtime_camera, Puzzle2dActionCtx};
use serde_json::Value;

pub fn set_camera(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(camera) = args.and_then(|value| value.get("camera")) else {
        return;
    };
    if let (Some(x), Some(y), Some(zoom)) = (camera.get("x").and_then(|value| value.as_f64()), camera.get("y").and_then(|value| value.as_f64()), camera.get("zoom").and_then(|value| value.as_f64())) {
        ctx.host.borrow_mut().set_camera(x, y, zoom);
    }
    set_runtime_camera(&mut ctx.scene.runtime, camera);
    *ctx.ui_scope = puzzle2d_window_only_scope();
}
