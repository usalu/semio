//! 🚀️ `translate-selection` command.

use crate::editor::puzzle2d::{Puzzle2dActionCtx, Puzzle2dSelectPhase, Puzzle2dSelectionRecord};
use serde_json::Value;

/// 🚀️ Moves every selected node and target region by `{dx, dy}` (world units; a `step` multiplies both, so the
/// arrow-key nudges send `{dx: ±1, dy: 0, step: <grid>}`) through the select tool: the `drag-selection` leaf over
/// the selection as literal targets, plus the `connect-handles` the drop lands where a moved open handle meets a
/// compatible one inside the window's `proximityRadius`. Without a `phase` one dispatch is one transaction, so three
/// nudges are three history rows; a host streaming a gesture sends `phase: "stream"` ticks that accumulate in the
/// window's ONE open transaction (previewed, never history) until `phase: "commit"` commits it or
/// `phase: "abort"` (with a `reason`) drops it with zero trace — the app's own answer to a host event
/// (`Puzzle2dPlayApp::host_event`), which no host sends itself.
pub fn translate_selection(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(phase) = Puzzle2dSelectPhase::from_args(args) else { return };
    let read = |key: &str| args.and_then(|value| value.get(key)).and_then(Value::as_f64).filter(|value| value.is_finite());
    let step = read("step").unwrap_or(1.0);
    let (dx, dy) = (read("dx").unwrap_or(0.0) * step, read("dy").unwrap_or(0.0) * step);
    let records = if dx == 0.0 && dy == 0.0 { Vec::new() } else { vec![Puzzle2dSelectionRecord::drag(ctx.selected_transform_targets(), dx, dy)] };
    if records.is_empty() && matches!(phase, Puzzle2dSelectPhase::Once | Puzzle2dSelectPhase::Stream) {
        return;
    }
    ctx.transform_selection("translateSelection", phase, records);
}
