//! 🗂️ `set-selection-flag` command.

use crate::editor::puzzle2d::{apply_selection_flag, Puzzle2dActionCtx};
use serde_json::Value;

/// 🙈️ An explicit `ids` list (the outliner tree's inline row toggles) patches exactly those; without
/// one the whole live selection is flagged at once (the context menu's and the inspector's path).
pub fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let explicit: Option<Vec<String>> = args
        .and_then(|value| value.get("ids"))
        .and_then(Value::as_array)
        .map(|ids| ids.iter().filter_map(|id| id.as_str().map(str::to_string)).collect::<Vec<String>>())
        .filter(|ids| !ids.is_empty());
    let ids = explicit.unwrap_or_else(|| ctx.selected_ids());
    apply_selection_flag(&mut ctx.scene.fixture, &ids, flag, value);
}
