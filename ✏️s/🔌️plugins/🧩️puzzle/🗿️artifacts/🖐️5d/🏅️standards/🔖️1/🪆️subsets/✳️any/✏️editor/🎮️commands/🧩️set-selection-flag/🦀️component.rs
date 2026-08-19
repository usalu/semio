//! 🧩️ `set-selection-flag` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

/// 👁️ Sets `hidden`/`locked` on every selected part.
pub async fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    let part_ids = ctx.selected_part_ids();
    for part in &mut ctx.scene.document.parts {
        if !part_ids.contains(&part.id) {
            continue;
        }
        match flag {
            "hidden" => part.part_2d.hidden = Some(value),
            "locked" => part.part_2d.locked = Some(value),
            _ => {}
        }
    }
}
