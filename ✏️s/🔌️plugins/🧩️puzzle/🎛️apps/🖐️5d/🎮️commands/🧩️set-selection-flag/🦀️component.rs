//! 🧩️ `set-selection-flag` command.

use crate::apps::puzzle5d::config::Puzzle5dSelection;
use crate::apps::puzzle5d::{add_palette_part, next_part_id, remove_grips, remove_parts, Puzzle5dActionCtx, Puzzle5dPart};
use semio_framework_plugin::SelectionSet;
use serde_json::{json, Value};

/// 👁️ Sets `hidden`/`locked` on every selected part.
pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    let part_ids = ctx.scene.runtime.selection.part_ids.clone();
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
