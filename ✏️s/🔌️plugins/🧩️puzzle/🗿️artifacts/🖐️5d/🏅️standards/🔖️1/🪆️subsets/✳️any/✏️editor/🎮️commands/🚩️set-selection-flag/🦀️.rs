//! 🚩️ `set-selection-flag` command.

use crate::editor::puzzle5d::{apply_puzzle5d_selection_flag, Puzzle5dActionCtx, PUZZLE5D_GRANULARITY_PART};
use dsl::os_pack::json::Value;

/// 👁️ Explicit `{entity, ids}` (the document tree's row actions) flags exactly those; otherwise the
/// whole live part selection is flagged at once (the context menu's and inspector's path).
pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
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
