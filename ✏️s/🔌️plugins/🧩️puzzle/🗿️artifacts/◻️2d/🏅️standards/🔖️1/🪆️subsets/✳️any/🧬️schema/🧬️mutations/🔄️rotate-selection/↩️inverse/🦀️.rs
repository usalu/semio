//! ↩️ Inverse for `RotateSelection` — the absolute setters restoring every BASE position and handle the
//! rotation turns (exact, never a negated angle that would accumulate float error). Nothing moved ⇒
//! `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_inverse, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RotateSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
    puzzle2d_selection_inverse(base, super::diff::diff(payload, base))?

    })
}
//#endregion 🔖️Inverse
