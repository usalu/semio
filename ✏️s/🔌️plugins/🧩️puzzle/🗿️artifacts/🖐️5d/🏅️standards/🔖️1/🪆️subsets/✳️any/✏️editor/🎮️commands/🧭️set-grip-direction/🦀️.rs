//! 🧭️ `set-grip-direction` command — how grip direction arrows are drawn.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_GRIP_DIRECTION_INWARDS, PUZZLE5D_GRIP_DIRECTION_OUTWARDS};
use dsl::os_pack::json::Value;

pub fn set_grip_direction(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(Value::as_str) {
        if mode == PUZZLE5D_GRIP_DIRECTION_OUTWARDS || mode == PUZZLE5D_GRIP_DIRECTION_INWARDS {
            ctx.scene.runtime.grip_direction = mode.into();
        }
    }
}
