//! 🗂️ `delete-selection` command.

use crate::editor::puzzle2d::{puzzle2d_clear_selection_write, Puzzle2dActionCtx};

/// 🗑️ Records the removal of the selected nodes (`delete-node`), handles (`remove-node-handle`), edges
/// (`disconnect-handles`) and target regions (`delete-target-region`) — each severing the edges hanging off it — and empties the
/// live selection through an app-initiated subtractive write, so nothing keeps pointing at ids the
/// document no longer has.
pub fn delete_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    if selected_ids.is_empty() {
        return;
    }
    if ctx.refuse_when_locked(&selected_ids) {
        return;
    }
    let clear = puzzle2d_clear_selection_write(&ctx.scene.board_snapshot, &selected_ids);
    ctx.recorder.delete_entities(&selected_ids);
    ctx.interaction_writes.extend(clear);
}
