//! 💡️ `close-handle-suggestions` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};

/// 🔒️ Discards the popup and the brush slot behind it without placing anything — escape, an
/// outside click, or the menu closing itself. The provisional preview dies with the slot.
pub fn close_handle_suggestions(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_cancel_slot();
    ctx.scene.runtime.suggestion_menu = None;
    ctx.scene.runtime.brush_candidates.clear();
    ctx.scene.runtime.brush_candidate_source_handle_id.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
