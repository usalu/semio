//! 🗂️ `delete-selection` command.

use crate::editor::puzzle2d::{delete_selection_from_host_snapshot, delete_target_regions_from_fixture, puzzle2d_clear_selection_write, puzzle2d_selected_target_region_ids, Puzzle2dActionCtx};

/// 🗑️ Removes the selected nodes, handles and edges (with the edges hanging off them) and empties the
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
    let clear = puzzle2d_clear_selection_write(&ctx.scene.fixture, &selected_ids);
    ctx.host.borrow_mut().delete_selection();
    delete_selection_from_host_snapshot(&mut ctx.scene.fixture, &selected_ids);
    let region_ids = puzzle2d_selected_target_region_ids(&ctx.scene.fixture, &selected_ids);
    delete_target_regions_from_fixture(&mut ctx.scene.fixture, &region_ids);
    ctx.interaction_writes.extend(clear);
}
