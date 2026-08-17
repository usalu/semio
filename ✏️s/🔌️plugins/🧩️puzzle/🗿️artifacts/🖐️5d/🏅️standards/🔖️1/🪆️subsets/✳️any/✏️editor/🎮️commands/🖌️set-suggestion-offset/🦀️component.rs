//! 🖌️ `set-suggestion-offset` command.

use serde_json::Value;
use crate::editor::puzzle5d::PUZZLE5D_SUGGESTION_OFFSET_MAX;
use crate::editor::puzzle5d::PUZZLE5D_SUGGESTION_OFFSET_MIN;
use crate::editor::puzzle5d::Puzzle5dActionCtx;

pub fn set_suggestion_offset(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(distance) = args.and_then(|value| value.get("distance").or_else(|| value.get("value"))).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
    }
}
