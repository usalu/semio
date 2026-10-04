//! ↩️ Inverse for `ScaleSelection` — the absolute setters restoring every BASE position, region corner
//! and region extent the scale moves (exact, never a reciprocal factor that would accumulate float
//! error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_inverse, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ScaleSelection, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
    puzzle2d_selection_inverse(base, super::diff::diff(payload, base))?

    })
}
//#endregion 🔖️Inverse
