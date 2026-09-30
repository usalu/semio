//! ↩️ Inverse for `DragSelection` — the absolute setters restoring every BASE position the drag moves
//! (exact, never a negated offset that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle2d_selection_inverse, Puzzle2dMutation};
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragSelection, base: &Puzzle2dSnapshot) -> Vec<Puzzle2dMutation> {
    puzzle2d_selection_inverse(base, super::diff::diff(payload, base))
}
//#endregion 🔖️Inverse
