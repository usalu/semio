//! 🚩️ `set-target-region-flag` command.

use crate::editor::puzzle2d::{apply_target_region_flag, puzzle2d_selected_target_region_ids, Puzzle2dActionCtx};
use serde_json::Value;

/// 🙈️ An explicit `id` (the outliner row toggle and the context menu) flags exactly that region;
/// without one the live selection's regions are flagged at once, matching `setSelectionFlag`'s shape.
pub fn set_target_region_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(Value::as_bool).unwrap_or(true);
    let explicit = args.and_then(|args| args.get("id")).and_then(Value::as_str).map(|id| vec![id.to_string()]);
    let ids = explicit.unwrap_or_else(|| {
        let selected = ctx.selected_ids();
        puzzle2d_selected_target_region_ids(&ctx.scene.fixture, &selected)
    });
    if ids.is_empty() {
        return;
    }
    apply_target_region_flag(&mut ctx.scene.fixture, &ids, flag, value);
}
