//! 💡️ `cycle-candidate` command.

use crate::editor::puzzle2d::{puzzle2d_restore_brush_slot, puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};

/// 🔁️ Walks the open slot's candidate page one step — `tab` forward, `shift+tab` back — over the
/// SAME slot the suggestions popup and the armed brush share. The host wraps the index and re-emits
/// the candidate page, so the scene's index follows without this arm guessing it.
pub fn cycle_candidate(ctx: &mut Puzzle2dActionCtx<'_>, forward: bool) {
    if puzzle2d_restore_brush_slot(ctx).is_none() {
        return;
    }
    ctx.host.borrow_mut().brush_cycle_candidate(forward);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
