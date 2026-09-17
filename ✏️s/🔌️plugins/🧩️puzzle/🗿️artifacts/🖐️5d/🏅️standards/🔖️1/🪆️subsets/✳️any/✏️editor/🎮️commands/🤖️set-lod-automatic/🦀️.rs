//! 🔭️ `set-lod-automatic` command — whether the world pane picks its detail level from the zoom.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_lod_automatic(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool).unwrap_or(!ctx.scene.runtime.lod_automatic);
}
