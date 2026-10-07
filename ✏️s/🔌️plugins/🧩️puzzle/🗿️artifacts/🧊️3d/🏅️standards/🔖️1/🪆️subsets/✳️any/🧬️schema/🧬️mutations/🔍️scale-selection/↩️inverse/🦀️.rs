//! ↩️ Inverse for `ScaleSelection` — the absolute setters restoring every BASE scale the scaling changes
//! (exact, never a reciprocal factor that would accumulate float error). Nothing scaled ⇒ `Vec::new()`.
use crate::standards::v1::subsets::any::schema::mutations::{puzzle3d_selection_inverse,Puzzle3dMutation};

use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ScaleSelection, base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok({
    puzzle3d_selection_inverse(base, super::diff::diff(payload, base))?

    })
}
//#endregion 🔖️Inverse
