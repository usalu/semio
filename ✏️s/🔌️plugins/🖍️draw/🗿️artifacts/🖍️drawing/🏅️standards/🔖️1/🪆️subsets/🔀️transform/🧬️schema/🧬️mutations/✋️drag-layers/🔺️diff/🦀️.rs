//! 🔺️ Sparse diff builder for `DragLayers` — every surviving layer's origin moves by the offset mapped into its parent's
//! axes, read off the BASE transform, so the leaf replays on any base.
use crate::mutations::{drawing_dragged_transform, drawing_selection_diff};
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::DragLayers, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    if !(payload.dx.is_finite() && payload.dy.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a drag offset must be finite", payload.targets.clone());
    }
    drawing_selection_diff(base, &payload.targets, |source, parent| drawing_dragged_transform(source, parent, [payload.dx, payload.dy]))
}
//#endregion 🔖️Diff
