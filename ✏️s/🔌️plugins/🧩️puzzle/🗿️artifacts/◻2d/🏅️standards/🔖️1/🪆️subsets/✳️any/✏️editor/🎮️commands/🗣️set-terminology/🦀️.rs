//! 🗣️ `set-terminology` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub fn set_terminology(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.terminology = value.into();
    }
}
