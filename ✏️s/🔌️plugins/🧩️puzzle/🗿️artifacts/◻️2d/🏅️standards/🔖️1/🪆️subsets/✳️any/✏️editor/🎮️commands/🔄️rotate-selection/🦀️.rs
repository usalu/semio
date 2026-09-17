//! 🔄️ `rotate-selection` command.

use crate::editor::puzzle2d::{puzzle2d_transform_selection, Puzzle2dActionCtx, Puzzle2dTransform};
use serde_json::Value;

/// 🔄️ Rotates the selected nodes by `angle` degrees (or `radians`) about the selection's centroid —
/// positions orbit the centroid and every handle angle turns with its node, so edges keep their geometry.
pub fn rotate_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let radians = args
        .and_then(|value| value.get("radians").and_then(Value::as_f64).or_else(|| value.get("angle").or_else(|| value.get("degrees")).and_then(Value::as_f64).map(f64::to_radians)))
        .filter(|value| value.is_finite() && *value != 0.0);
    let Some(radians) = radians else { return };
    let selected_ids = ctx.selected_ids();
    if ctx.refuse_when_locked(&selected_ids) {
        return;
    }
    puzzle2d_transform_selection(&mut ctx.scene.fixture, &selected_ids, Puzzle2dTransform::Rotate { radians });
}
