//! 🗂️ `set-selection-flag` command.

use crate::editor::puzzle2d::{apply_selection_flag, Puzzle2dActionCtx};
use serde_json::Value;

pub async fn set_selection_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("hidden");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(true);
    let selected_ids = ctx.selected_ids();
    apply_selection_flag(&mut ctx.scene.fixture, &selected_ids, flag, value);
}
