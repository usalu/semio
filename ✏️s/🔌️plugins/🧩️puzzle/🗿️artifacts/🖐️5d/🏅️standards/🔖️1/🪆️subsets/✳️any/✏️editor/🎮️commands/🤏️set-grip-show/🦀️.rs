//! 🤏️ `set-grip-show` command — when the world pane emits grip markers.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_GRIP_SHOW_ALWAYS, PUZZLE5D_GRIP_SHOW_SELECTED};
use dsl::os_pack::json::Value;

pub fn set_grip_show(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(Value::as_str) {
        if mode == PUZZLE5D_GRIP_SHOW_ALWAYS || mode == PUZZLE5D_GRIP_SHOW_SELECTED {
            ctx.scene.runtime.grip_show = mode.into();
        }
    }
}
