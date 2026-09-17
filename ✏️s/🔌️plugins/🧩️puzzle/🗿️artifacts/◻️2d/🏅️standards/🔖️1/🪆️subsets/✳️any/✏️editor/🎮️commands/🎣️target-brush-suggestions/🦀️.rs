//! 💡️ `target-brush-suggestions` command.

use crate::editor::puzzle2d::modes::edit::windows::overview::utilities::brush;
use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 🎣️ Points the candidate search at the handle the ARMED brush is over — or at nothing once the
/// brush is disarmed or the pointer left every handle. It only moves the slot: the candidate page it
/// resolves is the same one the suggestions popup reads, which is what keeps the two entry points on
/// one mechanism. A disarmed brush always clears, so a stale target can never survive a tool switch.
pub fn target_brush_suggestions(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let armed = ctx.scene.active_utility == brush::UTILITY_ID;
    let requested = args.and_then(|value| value.get("handleId")).and_then(Value::as_str).filter(|id| !id.is_empty());
    let target = armed.then_some(requested).flatten();
    ctx.host.borrow_mut().brush_target_slot(target);
    if target.is_none() {
        ctx.scene.runtime.brush_candidates.clear();
        ctx.scene.runtime.brush_candidate_source_handle_id.clear();
        ctx.scene.runtime.brush_candidate_index = 0;
    }
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
