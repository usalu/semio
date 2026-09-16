//! 🚀️ `translate-selection` command.

use crate::editor::puzzle2d::{puzzle2d_transform_selection, Puzzle2dActionCtx, Puzzle2dTransform};
use serde_json::Value;

/// 🚀️ Moves every selected node by `{dx, dy}` (world units; a `step` multiplies both, so the arrow-key
/// nudges send `{dx: ±1, dy: 0, step: <grid>}`).
pub fn translate_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64).filter(|value| value.is_finite());
    let step = read("step").unwrap_or(1.0);
    let (dx, dy) = (read("dx").unwrap_or(0.0) * step, read("dy").unwrap_or(0.0) * step);
    if dx == 0.0 && dy == 0.0 {
        return;
    }
    let selected_ids = ctx.selected_ids();
    puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Translate { dx, dy });
}
