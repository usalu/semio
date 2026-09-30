//! 🔺️ Sparse diff builder for `ScaleLayers` — every surviving layer scales about the world pivot through its parent's
//! world matrix, read off the BASE transform; a zero factor is refused as the schema states.
use crate::mutations::{drawing_moved_transform, drawing_scaling_matrix, drawing_selection_diff};
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::ScaleLayers, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    if ![payload.pivot_x, payload.pivot_y, payload.scale_x, payload.scale_y].iter().all(|value| value.is_finite()) || payload.scale_x == 0.0 || payload.scale_y == 0.0 {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a scaling pivot must be finite and both factors finite and nonzero", payload.targets.clone());
    }
    let motion = drawing_scaling_matrix(payload.pivot_x, payload.pivot_y, payload.scale_x, payload.scale_y);
    let identity = payload.scale_x == 1.0 && payload.scale_y == 1.0;
    drawing_selection_diff(base, &payload.targets, |source, parent| if identity { Some(source.clone()) } else { drawing_moved_transform(source, parent, motion) })
}
//#endregion 🔖️Diff
