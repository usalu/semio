//! 🖍️ `set-area-brush-size` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 🖍️ One Area Brush extent stepper — `{axis: "w"|"h", value}` in whole grid cells, clamped to at
/// least one cell. The 2d twin of puzzle3d's `setVoxelDims`: window-config state only, never a
/// document edit.
pub fn set_area_brush_size(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let axis = args.and_then(|value| value.get("axis")).and_then(Value::as_str).unwrap_or("");
    let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64).filter(|value| value.is_finite()) else {
        return;
    };
    let extent = value.max(1.0).round();
    match axis {
        "w" => ctx.scene.runtime.area_brush_width = extent,
        "h" => ctx.scene.runtime.area_brush_height = extent,
        _ => return,
    }
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
