//! 🗂️ `set-selection-flag` command, and the two set-verbs an outliner row names (`setSelectionHidden`,
//! `setSelectionLocked`).

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework_pack_json::Value;

/// 🙈️ An explicit `ids` list (the outliner tree's inline row toggles) patches exactly those; without
/// one the whole live selection is flagged at once (the context menu's and the inspector's path).
pub fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let explicit: Option<Vec<String>> = args
        .and_then(|value| value.get("ids"))
        .and_then(Value::as_array)
        .map(|ids| ids.iter().filter_map(|id| id.as_str().map(str::to_string)).collect::<Vec<String>>())
        .filter(|ids| !ids.is_empty());
    let ids = explicit.unwrap_or_else(|| ctx.selected_ids());
    ctx.recorder.set_flag(&ids, flag, value);
}
