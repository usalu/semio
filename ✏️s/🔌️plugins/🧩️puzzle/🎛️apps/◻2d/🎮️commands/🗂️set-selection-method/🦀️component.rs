//! 🗂️ `set-selection-method` command.

use crate::apps::puzzle2d::{apply_selection_flag, delete_selection_from_fixture, duplicate_selection_in_fixture, fixture_nodes, puzzle2d_select_scope, puzzle2d_window_only_scope, select_same_kind_ids, selection_ids, Puzzle2dActionCtx};
use serde_json::Value;

pub fn set_selection_method(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
    ctx.scene.runtime.selection_method = method.into();
    ctx.host.borrow_mut().set_selection_options(method, "replace", true, true, true);
    *ctx.ui_scope = puzzle2d_window_only_scope();
}
