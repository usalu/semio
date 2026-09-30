//! 🔺️ Sparse diff builder for `RotateLayers` — every surviving layer turns about the world pivot through its parent's
//! world matrix, read off the BASE transform.
use crate::mutations::{drawing_moved_transform, drawing_rotation_matrix, drawing_selection_diff};
use crate::DrawingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::mutation::RotateLayers, base: &DrawingSnapshot) -> protocol::MutationOutcome<crate::diff::DrawingDiff> {
    if !(payload.pivot_x.is_finite() && payload.pivot_y.is_finite() && payload.angle.is_finite()) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "a rotation pivot and angle must be finite", payload.targets.clone());
    }
    let motion = drawing_rotation_matrix(payload.pivot_x, payload.pivot_y, payload.angle);
    drawing_selection_diff(base, &payload.targets, |source, parent| if payload.angle == 0.0 { Some(source.clone()) } else { drawing_moved_transform(source, parent, motion) })
}
//#endregion 🔖️Diff
