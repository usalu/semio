//! 🖌️ `cycle-candidate` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub async fn cycle_candidate(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let forward = args.and_then(|value| value.get("forward")).and_then(|value| value.as_bool()).unwrap_or(true);
    ctx.host.borrow_mut().brush_cycle_candidate(forward);
    ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add(1);
    *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
}
