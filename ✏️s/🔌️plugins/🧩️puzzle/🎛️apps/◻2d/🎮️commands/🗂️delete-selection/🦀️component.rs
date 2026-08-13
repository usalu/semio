//! 🗂️ `delete-selection` command.

use crate::apps::puzzle2d::{apply_selection_flag, delete_selection_from_fixture, duplicate_selection_in_fixture, fixture_nodes, puzzle2d_select_scope, puzzle2d_window_only_scope, select_same_kind_ids, selection_ids, Puzzle2dActionCtx};
use serde_json::Value;

pub fn delete_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().delete_selection();
    delete_selection_from_fixture(&mut ctx.scene.fixture, &ctx.scene.runtime.selected_ids);
    ctx.scene.runtime.selected_ids.clear();
}
