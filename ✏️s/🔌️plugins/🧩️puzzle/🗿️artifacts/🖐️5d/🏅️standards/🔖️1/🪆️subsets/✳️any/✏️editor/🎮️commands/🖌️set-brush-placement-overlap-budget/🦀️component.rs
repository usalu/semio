//! 🖌️ `set-brush-placement-overlap-budget` command.

use serde_json::Value;
use crate::editor::puzzle5d::Puzzle5dActionCtx;

pub async fn set_brush_placement_overlap_budget(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()) {
        ctx.scene.runtime.overlap_budget = value.clamp(0.0, 1.0);
        ctx.app.drive_precompute(ctx.scene);
    }
}
