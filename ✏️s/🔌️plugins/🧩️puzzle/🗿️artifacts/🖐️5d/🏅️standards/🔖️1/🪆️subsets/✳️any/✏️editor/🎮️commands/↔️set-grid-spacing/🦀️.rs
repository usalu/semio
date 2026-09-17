//! 🌐️ `set-grid-spacing` command — the world pane's grid pitch (m), the same spacing snapping rounds onto.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_GRID_SPACING_MAX, PUZZLE5D_GRID_SPACING_MIN};
use dsl::os_pack::json::Value;

pub fn set_grid_spacing(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
        ctx.scene.runtime.grid_spacing = value.clamp(PUZZLE5D_GRID_SPACING_MIN, PUZZLE5D_GRID_SPACING_MAX);
    }
}
