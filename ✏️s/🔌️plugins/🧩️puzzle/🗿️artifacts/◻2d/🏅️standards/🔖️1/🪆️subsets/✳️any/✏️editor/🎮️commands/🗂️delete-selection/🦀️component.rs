//! 🗂️ `delete-selection` command.

use crate::editor::puzzle2d::{delete_selection_from_fixture, Puzzle2dActionCtx};

pub fn delete_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let selected_ids = ctx.selected_ids();
    ctx.host.borrow_mut().delete_selection();
    delete_selection_from_fixture(&mut ctx.scene.fixture, &selected_ids);
}
