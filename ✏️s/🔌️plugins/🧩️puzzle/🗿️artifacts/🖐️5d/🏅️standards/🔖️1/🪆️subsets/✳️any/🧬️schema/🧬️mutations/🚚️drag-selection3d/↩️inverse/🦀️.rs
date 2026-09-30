//! ↩️ Inverse for `DragSelection3d` — the absolute setters restoring every BASE pose field the transform changes (exact,
//! never a negated parameter that would accumulate float error). Nothing moved ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle5d_selection_inverse, Puzzle5dMutation};
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DragSelection3d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    puzzle5d_selection_inverse(base, super::diff::diff(payload, base))
}
//#endregion 🔖️Inverse
