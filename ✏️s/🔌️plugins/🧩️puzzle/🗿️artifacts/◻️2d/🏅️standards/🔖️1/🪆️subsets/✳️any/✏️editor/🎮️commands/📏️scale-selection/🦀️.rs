//! 📏️ `scale-selection` command.

use crate::editor::puzzle2d::{puzzle2d_gesture_phase, puzzle2d_selection_pivot, Puzzle2dActionCtx, Puzzle2dSelectionMotion, Puzzle2dSelectionRecord};
use semio_framework_tool_machine::GesturePhase;
use serde_json::Value;

/// 📏️ Scales the selected nodes' positions and target regions about their joint centroid by `factor` through the
/// select tool (node sizes stay — a node kind's footprint is the kind's, not the layout's; a region scales its
/// corner AND its extent, having no kind catalogue to take a footprint from). The `scale-selection` leaf records the
/// ids, the one pivot and the factor; `phase` streams, commits or aborts a multi-dispatch scaling whose ticks
/// multiply about one pivot.
pub fn scale_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(phase) = puzzle2d_gesture_phase(args) else { return };
    let factor = args.and_then(|value| value.get("factor").or_else(|| value.get("value"))).and_then(Value::as_f64).filter(|factor| factor.is_finite() && *factor > 0.0 && *factor != 1.0);
    let targets = ctx.selected_transform_targets();
    let records = match (factor, puzzle2d_selection_pivot(ctx.base.typed(), &targets, true)) {
        (Some(factor), Some((pivot_x, pivot_y))) => vec![Puzzle2dSelectionRecord { targets, motion: Puzzle2dSelectionMotion::Scale { pivot_x, pivot_y, factor }, proximity: Vec::new(), connect: false }],
        _ => Vec::new(),
    };
    if records.is_empty() && matches!(phase, GesturePhase::Once | GesturePhase::Stream) {
        return;
    }
    ctx.transform_selection("scaleSelection", phase, records);
}
