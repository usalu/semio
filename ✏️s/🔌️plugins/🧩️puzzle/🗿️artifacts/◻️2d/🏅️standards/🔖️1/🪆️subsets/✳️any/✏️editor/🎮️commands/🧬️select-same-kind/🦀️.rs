//! 🗂️ `select-same-kind` command.

use crate::editor::puzzle2d::{puzzle2d_selection_write, select_same_kind_ids, Puzzle2dActionCtx};
use semio_framework::kernel::UiDirtyScope;

/// 🧬️ Widens the live `vortex` selection to every node sharing a kind with a selected node — an
/// app-initiated selection write (`Emit::interaction_writes`), never a document operation.
pub fn select_same_kind(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    let widened = select_same_kind_ids(&ctx.scene.fixture, &selected_ids);
    if widened.is_empty() {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    }
    ctx.interaction_writes.push(puzzle2d_selection_write(&ctx.scene.fixture, &widened));
    *ctx.ui_scope = crate::editor::puzzle2d::puzzle2d_select_scope();
}
