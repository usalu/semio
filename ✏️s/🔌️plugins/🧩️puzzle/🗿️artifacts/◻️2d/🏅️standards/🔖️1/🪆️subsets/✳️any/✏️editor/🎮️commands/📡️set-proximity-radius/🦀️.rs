//! 📡️ `set-proximity-radius` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx, PUZZLE2D_PROXIMITY_RADIUS_MAX};
use serde_json::Value;

/// 📡️ Retargets the board units within which a dropped node auto-connects. An absolute `value`
/// (or `radius`) sets it, a `delta` nudges the live one — the shape the settings stepper sends.
pub fn set_proximity_radius(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64).filter(|value| value.is_finite());
    let Some(next) = read("value").or_else(|| read("radius")).or_else(|| read("delta").map(|delta| ctx.scene.runtime.proximity_radius + delta)) else {
        return;
    };
    ctx.scene.runtime.proximity_radius = next.clamp(0.0, PUZZLE2D_PROXIMITY_RADIUS_MAX);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
