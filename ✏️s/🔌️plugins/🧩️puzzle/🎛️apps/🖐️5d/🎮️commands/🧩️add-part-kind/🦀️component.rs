//! 🧩️ `add-part-kind` command.

use crate::apps::puzzle5d::config::Puzzle5dSelection;
use crate::apps::puzzle5d::{add_palette_part, next_part_id, remove_grips, remove_parts, Puzzle5dActionCtx, Puzzle5dPart};
use semio_framework_plugin::SelectionSet;
use serde_json::{json, Value};

/// 🛍️ Catalogue placement — routed through the paired board/engine brush placement so both aspects land at once.
pub fn add_part_kind(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = args.and_then(|value| value.get("partKind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let payload = json!({ "nodeKind": part_kind, "x": 120.0, "y": 120.0 });
    ctx.app.apply_board_brush_place(ctx.scene, &payload);
}
