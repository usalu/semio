//! ↩️ Inverse for `DragLayers` — the absolute transforms of every layer the drag moves, captured from BASE.
use crate::mutations::{drawing_selection_inverse, DrawingMutation};
use crate::DrawingSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DragLayers, base: &DrawingSnapshot) -> Vec<DrawingMutation> {
    drawing_selection_inverse(base, super::diff::diff(payload, base))
}
//#endregion 🔖️Inverse
