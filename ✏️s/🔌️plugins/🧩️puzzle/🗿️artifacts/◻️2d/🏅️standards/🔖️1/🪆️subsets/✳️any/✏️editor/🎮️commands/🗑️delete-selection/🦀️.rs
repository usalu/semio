//! 🗂️ `delete-selection` command.

use crate::editor::puzzle2d::{delete_selection_from_host_snapshot, puzzle2d_selection_write, Puzzle2dActionCtx};

/// 🗑️ Removes the selected nodes (with their edges) and clears the live selection through an
/// app-initiated selection write, so nothing keeps pointing at ids the document no longer has.
pub fn delete_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    if selected_ids.is_empty() {
        return;
    }
    ctx.host.borrow_mut().delete_selection();
    delete_selection_from_host_snapshot(&mut ctx.scene.fixture, &selected_ids);
    ctx.interaction_writes.push(puzzle2d_selection_write(&ctx.scene.fixture, &[]));
}
