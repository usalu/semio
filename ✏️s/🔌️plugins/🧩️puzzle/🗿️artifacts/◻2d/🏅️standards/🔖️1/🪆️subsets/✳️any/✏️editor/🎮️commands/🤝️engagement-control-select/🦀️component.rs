//! 🤝️ `engagement-control-select` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub fn engagement_control_select(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let candidate_id = args.and_then(|value| value.get("id").or_else(|| value.get("value"))).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(index) = candidate_id.strip_prefix("puzzle2d.brush.candidate.").and_then(|rest| rest.parse::<usize>().ok()) {
        ctx.host.borrow_mut().brush_set_candidate_index(index);
        ctx.scene.runtime.brush_candidate_index = index;
    }
}
