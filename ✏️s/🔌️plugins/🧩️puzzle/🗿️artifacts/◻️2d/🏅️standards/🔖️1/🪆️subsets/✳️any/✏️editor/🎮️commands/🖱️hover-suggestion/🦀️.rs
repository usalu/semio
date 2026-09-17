//! 💡️ `hover-suggestion` command.

use crate::editor::puzzle2d::{puzzle2d_restore_brush_slot, puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 🖱️ Previews one candidate PROVISIONALLY — the board host rebuilds its ghost on `index` and nothing
/// reaches the document until `acceptSuggestion` commits it. Hovering a popup row and cycling the armed
/// brush write the same index, so "just looking" is one state, not two.
pub fn hover_suggestion(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) else {
        return;
    };
    let requested = args.and_then(|value| value.get("handleId")).and_then(|value| value.as_str()).map(str::to_string);
    if puzzle2d_restore_brush_slot(ctx, requested.as_deref()).is_none() {
        return;
    }
    ctx.host.borrow_mut().brush_set_candidate_index(index as usize);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
