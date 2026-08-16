//! 🗣️ `set-locale` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub fn set_locale(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        ctx.scene.runtime.locale = value.into();
    }
}
