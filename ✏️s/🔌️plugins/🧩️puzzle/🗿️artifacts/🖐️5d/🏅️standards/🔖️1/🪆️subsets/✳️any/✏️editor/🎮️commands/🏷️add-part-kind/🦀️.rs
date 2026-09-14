//! 🏷️ `add-part-kind` command.

use crate::editor::puzzle5d::commands::add_brush_part::puzzle5d_place_brush_part;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🛍️ Catalogue placement — the shared brush placement, so both aspects land at once.
pub fn add_part_kind(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = args.and_then(|value| value.get("partKind")).and_then(Value::as_str).unwrap_or("Part").to_string();
    puzzle5d_place_brush_part(ctx, &part_kind, None, [Some(120.0), Some(120.0)], None, None);
}
