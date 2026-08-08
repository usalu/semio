//! ↩️ Inverse for `SetStroke`.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawDocument;

//#region 🔖️Inverse
/// Draw inverses snapshot the pre-state document (exact restore).
pub fn inverse(base: &DrawDocument) -> Vec<DrawMutation> {
    vec![DrawMutation::SetDocument { document: base.clone() }]
}
//#endregion 🔖️Inverse
