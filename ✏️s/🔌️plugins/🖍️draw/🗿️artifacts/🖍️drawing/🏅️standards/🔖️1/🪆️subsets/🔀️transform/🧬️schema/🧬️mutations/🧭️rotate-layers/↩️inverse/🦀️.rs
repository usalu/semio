//! ↩️ Inverse for `RotateLayers` — the absolute transforms of every layer the rotation turns, captured from BASE.
use crate::mutations::{drawing_selection_inverse, DrawingMutation};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RotateLayers, base: &DrawingSnapshot) -> Result<Vec<DrawingMutation>, semio_framework_value::ValueError> {
    Ok({
    drawing_selection_inverse(base, super::diff::diff(payload, base))?

    })
}
//#endregion 🔖️Inverse
