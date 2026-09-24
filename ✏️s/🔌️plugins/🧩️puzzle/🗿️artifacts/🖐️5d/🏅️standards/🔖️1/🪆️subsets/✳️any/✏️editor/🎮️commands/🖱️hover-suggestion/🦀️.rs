//! 🖱️ `hover-suggestion` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🖱️ The suggestion row under the pointer becomes the candidate an argument-less accept or cycle starts from.
pub fn hover_suggestion(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(Value::as_u64) {
        ctx.scene.runtime.brush_candidate_index = index as usize;
    }
}
