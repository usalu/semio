//! ⚙️ `set-brush-placement-contact-tolerance` command — the world-unit margin every brush/fill
//! placement footprint is grown by before the collision test, a shared `Puzzle2dConfig` preference a
//! live fill run re-reads through `ToolRunSettingsReads`.

use crate::editor::puzzle2d::{puzzle2d_absolute_or_delta, puzzle2d_window_and_measures_scope, Puzzle2dActionCtx, PUZZLE2D_PLACEMENT_MEASURE_MAX};
use serde_json::Value;

pub fn set_brush_placement_contact_tolerance(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle2d_absolute_or_delta(args, ctx.scene.runtime.contact_tolerance) {
        ctx.scene.runtime.contact_tolerance = value.clamp(0.0, PUZZLE2D_PLACEMENT_MEASURE_MAX);
        *ctx.ui_scope = puzzle2d_window_and_measures_scope();
    }
}
