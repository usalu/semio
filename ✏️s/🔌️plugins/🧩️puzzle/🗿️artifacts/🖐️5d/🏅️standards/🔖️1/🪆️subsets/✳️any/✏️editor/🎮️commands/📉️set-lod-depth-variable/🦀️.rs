//! 🔭️ `set-lod-depth-variable` command — whether detail falls off with distance from the camera.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_lod_depth_variable(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool).unwrap_or(!ctx.scene.runtime.lod_depth_variable);
}
