//! ↩️ Inverse for `DuplicateLayer`.
use crate::artifacts::draw::mutations::DrawMutation;
use crate::artifacts::draw::DrawSnapshot;

//#region 🔖️Inverse
/// Draw inverses snapshot the pre-state document (exact restore).
pub fn inverse(base: &DrawSnapshot) -> Vec<DrawMutation> {
    vec![DrawMutation::SetSnapshot { snapshot: base.clone() }]
}
//#endregion 🔖️Inverse
