//! 🗂️ `duplicate-selection` command.

use crate::editor::puzzle2d::{puzzle2d_selection_write, Puzzle2dActionCtx};

/// 👯️ Records a `create-node` per selected node (an offset copy with a fresh id, stamped with the next free display label
/// for its kind) and a `connect-handles` per edge between the copies, and re-selects the clones through an
/// app-initiated selection write, so the next gesture acts on what was just created.
pub fn duplicate_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    let clones = ctx.recorder.duplicate(&selected_ids);
    if !clones.is_empty() {
        let write = puzzle2d_selection_write(ctx.recorder.value(), &clones);
        ctx.interaction_writes.push(write);
    }
}
