//! ↩️ Inverse for `DeleteReference` — reconstructs a `create-reference` of the captured
//! BASE entry. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteReference, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    let index = base.references.iter().position(|entry| entry.id == payload.id);
    vec![crate::artifacts::puzzle3d::mutations::create_reference::mutation::create_reference(item.clone(), index)]
}
//#endregion 🔖️Inverse
