//! 📍️ placement may spend before it counts as a collision, a shared `Puzzle2dConfig` preference a live
//! ⚙️ `set-brush-placement-overlap-budget` command — the world-unit footprint overlap a brush/fill
//! fill run re-reads through `ToolRunSettingsReads`.

use {crate::editor::puzzle2d::snapshot::puzzle2d_absolute_or_delta, crate::editor::puzzle2d::snapshot::puzzle2d_window_and_measures_scope, crate::editor::puzzle2d::interaction::Puzzle2dActionCtx, crate::editor::puzzle2d::snapshot::PUZZLE2D_PLACEMENT_MEASURE_MAX};
use semio_framework_pack_json::Value;

pub fn set_brush_placement_overlap_budget(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle2d_absolute_or_delta(args, ctx.scene.runtime.brush_placement_overlap_budget) {
        ctx.scene.runtime.brush_placement_overlap_budget = value.clamp(0.0, PUZZLE2D_PLACEMENT_MEASURE_MAX);
        *ctx.ui_scope = puzzle2d_window_and_measures_scope();
    }
}
