//! 🚩️ `set-selection-flag` command, and the two set-verbs a document-tree part row names (`setSelectionHidden`,
//! `setSelectionLocked`).

use crate::editor::puzzle5d::{apply_puzzle5d_selection_flag, Puzzle5dActionCtx, PUZZLE5D_GRANULARITY_PART};
use dsl::os_pack::json::Value;

/// 👁️ Explicit `{entity, ids}` (the document tree's row actions) flags exactly those; otherwise the
/// whole live part selection is flagged at once (the context menu's and inspector's path).
pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setSelectionHidden{hidden}`/`setSelectionLocked{locked}`: `flag` set to exactly the boolean the arguments carry —
/// the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused a
/// missing value, so there is no default to fall back to here.
pub fn set_selection_flag_value(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let entity = args.and_then(|value| value.get("entity")).and_then(|value| value.as_str());
    let explicit_ids: Option<Vec<String>> = args.and_then(|value| value.get("ids")).and_then(|value| dsl::FromValue::from_value(dsl::os_pack::json::to_dsl_value(value)).ok());
    match (entity, explicit_ids) {
        (Some(entity), Some(ids)) => apply_puzzle5d_selection_flag(&mut ctx.scene.document, entity, &ids, flag, value),
        _ => {
            let part_ids = ctx.selected_part_ids();
            apply_puzzle5d_selection_flag(&mut ctx.scene.document, PUZZLE5D_GRANULARITY_PART, &part_ids, flag, value);
        }
    }
}
