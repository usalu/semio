//! 🔄️ `rotate-selection` command.

use crate::editor::puzzle2d::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};
use semio_framework_tool_machine::GesturePhase;
use semio_framework_pack_json::Value;

/// 🔄️ Rotates the selected nodes by `angle` degrees (or `radians`) about their centroid through the select tool:
/// the `rotate-selection` leaf records the ids, the pivot and the angle, so positions orbit the recorded pivot and
/// every handle angle turns with its node on any base the edit replays on. `phase` streams, commits or aborts a
/// multi-dispatch rotation exactly as `translateSelection` does; its ticks add up about one pivot.
pub fn rotate_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(phase) = puzzle2d_gesture_phase(args) else { return };
    let radians = args
        .and_then(|value| value.get("radians").and_then(Value::as_f64).or_else(|| value.get("angle").or_else(|| value.get("degrees")).and_then(Value::as_f64).map(f64::to_radians)))
        .filter(|value| value.is_finite() && *value != 0.0);
    let document = ctx.base;
    let base = document.typed();
    let targets: Vec<String> = ctx.selected_transform_targets().into_iter().filter(|id| base.nodes.iter().any(|node| &node.id == id)).collect();
    let records = match (radians, puzzle2d_selection_pivot(base, &targets, false)) {
        (Some(angle), Some((pivot_x, pivot_y))) => vec![Puzzle2dSelectionRecord { targets, motion: Puzzle2dSelectionMotion::Rotate { pivot_x, pivot_y, angle }, proximity: Vec::new(), connect: false }],
        _ => Vec::new(),
    };
    if records.is_empty() && matches!(phase, GesturePhase::Once | GesturePhase::Stream) {
        return;
    }
    ctx.transform_selection("rotateSelection", phase, records);
}
