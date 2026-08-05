//! 🗂️ Puzzle 2d play app commands — the selection vocabulary: setting/clearing/growing the selection,
//! the hidden/locked flag toggles, the marquee method, and the destructive delete/duplicate verbs.

use crate::apps::puzzle2d::{apply_selection_flag, delete_selection_from_fixture, duplicate_selection_in_fixture, fixture_nodes, puzzle2d_select_scope, puzzle2d_window_only_scope, select_same_kind_ids, selection_ids, Puzzle2dActionCtx};
use serde_json::Value;

pub fn set_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.selected_ids = selection_ids(args);
    ctx.host.borrow_mut().set_selection_ids(&ctx.scene.runtime.selected_ids);
    *ctx.ui_scope = puzzle2d_select_scope();
}

pub fn select_all(ctx: &mut Puzzle2dActionCtx<'_>) {
    let ids: Vec<String> = fixture_nodes(&ctx.scene.fixture).iter().filter_map(|node| node.get("id").and_then(|value| value.as_str()).map(str::to_string)).collect();
    ctx.scene.runtime.selected_ids = ids.clone();
    ctx.host.borrow_mut().set_selection_ids(&ids);
    *ctx.ui_scope = puzzle2d_select_scope();
}

pub fn clear_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.scene.runtime.selected_ids.clear();
    ctx.host.borrow_mut().set_selection_ids(&[]);
    *ctx.ui_scope = puzzle2d_select_scope();
}

pub fn select_same_kind(ctx: &mut Puzzle2dActionCtx<'_>) {
    let ids = select_same_kind_ids(&ctx.scene.fixture, &ctx.scene.runtime.selected_ids);
    if ids.is_empty() {
        return;
    }
    ctx.scene.runtime.selected_ids = ids;
    ctx.host.borrow_mut().set_selection_ids(&ctx.scene.runtime.selected_ids);
}

pub fn delete_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().delete_selection();
    delete_selection_from_fixture(&mut ctx.scene.fixture, &ctx.scene.runtime.selected_ids);
    ctx.scene.runtime.selected_ids.clear();
}

pub fn duplicate_selection(ctx: &mut Puzzle2dActionCtx<'_>) {
    let new_ids = duplicate_selection_in_fixture(&mut ctx.scene.fixture, &ctx.scene.runtime.selected_ids);
    if new_ids.is_empty() {
        return;
    }
    ctx.scene.runtime.selected_ids = new_ids;
    ctx.host.borrow_mut().set_selection_ids(&ctx.scene.runtime.selected_ids);
}

pub fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply_selection_flag(&mut ctx.scene.fixture, &ctx.scene.runtime.selected_ids, flag, value);
}

pub fn set_selection_method(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
    ctx.scene.runtime.selection_method = method.into();
    ctx.host.borrow_mut().set_selection_options(method, "replace", true, true, true);
    *ctx.ui_scope = puzzle2d_window_only_scope();
}
