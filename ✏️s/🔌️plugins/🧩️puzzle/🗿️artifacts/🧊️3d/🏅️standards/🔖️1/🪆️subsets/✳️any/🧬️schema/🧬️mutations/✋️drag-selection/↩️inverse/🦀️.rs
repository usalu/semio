//! ↩️ Inverse for `DragSelection` — the absolute setters restoring every BASE origin the drag moves
//! (exact, never a negated offset that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_inverse, Puzzle3dMutation};
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DragSelection, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    puzzle3d_selection_inverse(base, super::diff::diff(payload, base))
}
//#endregion 🔖️Inverse
