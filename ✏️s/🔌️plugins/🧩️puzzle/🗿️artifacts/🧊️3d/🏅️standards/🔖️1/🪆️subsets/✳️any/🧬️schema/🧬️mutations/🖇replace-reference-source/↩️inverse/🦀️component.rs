//! ↩️ Inverse for `ReplaceReferenceSource` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplaceReferenceSource, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle3d::mutations::replace_reference_source::mutation::replace_reference_source(item.id.clone(), item.source.clone())]
}
//#endregion 🔖️Inverse
