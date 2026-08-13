//! 🗂️ `select-all` command.

use crate::apps::puzzle2d::{apply_selection_flag, delete_selection_from_fixture, duplicate_selection_in_fixture, fixture_nodes, puzzle2d_select_scope, puzzle2d_window_only_scope, select_same_kind_ids, selection_ids, Puzzle2dActionCtx};
use serde_json::Value;

pub fn select_all(ctx: &mut Puzzle2dActionCtx<'_>) {
    let ids: Vec<String> = fixture_nodes(&ctx.scene.fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    ctx.scene.runtime.selected_ids = ids.clone();
    ctx.host.borrow_mut().set_selection_ids(&ids);
    *ctx.ui_scope = puzzle2d_select_scope();
}
