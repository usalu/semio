//! 🖌️ `engagement-control-select` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🎚️ The brush placement picker's select — its option values are `puzzle5d.brush.candidate.<index>`.
pub fn engagement_control_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle5d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.scene.runtime.brush_candidate_index = index;
    }
}
