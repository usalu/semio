//! 🔭️ `set-lod-mode` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

pub fn set_lod_mode(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value").or_else(|| value.get("mode"))).and_then(|value| value.as_str()) {
        ctx.scene.runtime.lod_mode = mode.into();
    }
}
