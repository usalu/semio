//! ↩️ Inverse for `ChangeReferenceLocked` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ChangeReferenceLocked, base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    let Some(item) = base.references.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_reference_locked::mutation::change_reference_locked(item.id.clone(), item.locked)]
}
//#endregion 🔖️Inverse
