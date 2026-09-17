//! 🚀️ `translate-selection` command.

use crate::editor::puzzle2d::commands::proximity_connect::puzzle2d_proximity_connect;
use crate::editor::puzzle2d::{puzzle2d_selected_target_region_ids, puzzle2d_transform_selection, puzzle2d_transform_target_regions, Puzzle2dActionCtx, Puzzle2dTransform};
use serde_json::Value;

/// 🚀️ Moves every selected node by `{dx, dy}` (world units; a `step` multiplies both, so the arrow-key
/// nudges send `{dx: ±1, dy: 0, step: <grid>}`). Where the move LANDS an open handle inside the
/// window's `proximityRadius` of a compatible open handle, the auto-connect rides the SAME edit, so
/// one undo takes the move and its new edges back together.
pub fn translate_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64).filter(|value| value.is_finite());
    let step = read("step").unwrap_or(1.0);
    let (dx, dy) = (read("dx").unwrap_or(0.0) * step, read("dy").unwrap_or(0.0) * step);
    if dx == 0.0 && dy == 0.0 {
        return;
    }
    let selected_ids = ctx.selected_ids();
    if ctx.refuse_when_locked(&selected_ids) {
        return;
    }
    puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Translate { dx, dy });
    let region_ids = puzzle2d_selected_target_region_ids(&ctx.scene.fixture, &selected_ids);
    puzzle2d_transform_target_regions(&mut ctx.scene.fixture, &region_ids, Puzzle2dTransform::Translate { dx, dy });
    let radius = ctx.scene.runtime.proximity_radius;
    puzzle2d_proximity_connect(&mut ctx.scene.fixture, &selected_ids, radius);
}
