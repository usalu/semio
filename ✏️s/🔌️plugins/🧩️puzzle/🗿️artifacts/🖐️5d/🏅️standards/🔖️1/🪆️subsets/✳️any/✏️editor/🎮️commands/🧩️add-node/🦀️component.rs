//! 🧩️ `add-node` command.

use crate::editor::puzzle5d::{add_palette_part, Puzzle5dActionCtx};
use serde_json::Value;

/// 🎨️ Palette drop at a flat point — the volume origin is derived from the nearest peer part.
pub async fn add_node(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
    add_palette_part(ctx.scene, &part_kind, x, y);
}
