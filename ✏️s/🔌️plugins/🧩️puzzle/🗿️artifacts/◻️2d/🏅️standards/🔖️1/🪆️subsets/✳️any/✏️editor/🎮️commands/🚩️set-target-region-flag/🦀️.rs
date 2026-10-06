//! 🚩️ `set-target-region-flag` command, and the two set-verbs an outliner region row names (`setTargetRegionHidden`,
//! `setTargetRegionLocked`).

use crate::editor::puzzle2d::{apply_target_region_flag, puzzle2d_selected_target_region_ids, Puzzle2dActionCtx};
use serde_json::Value;

/// 🙈️ An explicit `id` (the outliner row toggle and the context menu) flags exactly that region;
/// without one the live selection's regions are flagged at once, matching `setSelectionFlag`'s shape.
pub fn set_target_region_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(true);
    apply(ctx, args, flag, value);
}

/// 🎯️ `setTargetRegionHidden{hidden}`/`setTargetRegionLocked{locked}`: `flag` set to exactly the boolean the arguments
/// carry — the row target's explicit next state, so replaying it changes nothing. `command_from_action` already refused
/// a missing value, so there is no default to fall back to here.
pub fn set_target_region_flag_value(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str) {
    if let Some(value) = args.and_then(|value| value.get(flag)).and_then(Value::as_bool) {
        apply(ctx, args, flag, value);
    }
}

fn apply(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>, flag: &str, value: bool) {
    let explicit = args.and_then(|args| args.get("id")).and_then(Value::as_str).map(|id| vec![id.to_string()]);
    let ids = explicit.unwrap_or_else(|| {
        let selected = ctx.selected_ids();
        puzzle2d_selected_target_region_ids(&ctx.scene.board_snapshot, &selected)
    });
    if ids.is_empty() {
        return;
    }
    apply_target_region_flag(&mut ctx.scene.board_snapshot, &ids, flag, value);
}
