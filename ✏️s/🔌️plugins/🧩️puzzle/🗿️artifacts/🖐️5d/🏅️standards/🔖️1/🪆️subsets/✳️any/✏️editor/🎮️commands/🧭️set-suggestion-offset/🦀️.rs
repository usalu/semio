//! 🧭️ `set-suggestion-offset` command.

use crate::editor::puzzle5d::puzzle5d_absolute_or_delta;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use crate::editor::puzzle5d::PUZZLE5D_SUGGESTION_OFFSET_MAX;
use crate::editor::puzzle5d::PUZZLE5D_SUGGESTION_OFFSET_MIN;
use dsl::os_pack::json::Value;

/// 🧭️ How far off a grip a suggestion is offered: an absolute `value` (the brush option's slider) or
/// a `delta` nudge, clamped into the band the slider itself declares.
pub fn set_suggestion_offset(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(distance) = puzzle5d_absolute_or_delta(args, ctx.scene.runtime.suggestion_offset) {
        ctx.scene.runtime.suggestion_offset = distance.clamp(PUZZLE5D_SUGGESTION_OFFSET_MIN, PUZZLE5D_SUGGESTION_OFFSET_MAX);
    }
}
