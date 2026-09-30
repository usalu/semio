//! ↩️ Inverse for `RotateSelection` — the absolute setters restoring every BASE orientation the turn
//! changes (exact, never a negated angle that would accumulate float error). Nothing turned ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_inverse, Puzzle3dMutation};
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RotateSelection, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    puzzle3d_selection_inverse(base, super::diff::diff(payload, base))
}
//#endregion 🔖️Inverse
