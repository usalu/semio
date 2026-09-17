//! 🗂️ `duplicate-selection` command.

use crate::editor::puzzle2d::{duplicate_selection_in_fixture, puzzle2d_relabel_nodes, puzzle2d_selection_write, Puzzle2dActionCtx};

/// 👯️ Clones the selected nodes (offset copies with fresh ids, each re-stamped with the next free
/// display label for its kind) and re-selects the clones through an app-initiated selection write, so
/// the next gesture acts on what was just created.
pub fn duplicate_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    let clones = duplicate_selection_in_fixture(&mut ctx.scene.fixture, &selected_ids);
    if !clones.is_empty() {
        puzzle2d_relabel_nodes(&mut ctx.scene.fixture, &clones);
        ctx.interaction_writes.push(puzzle2d_selection_write(&ctx.scene.fixture, &clones));
    }
}
