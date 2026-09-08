//! ↩️ Inverse for `ChangePartAnchor` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangePartAnchor, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_part_anchor::change_part_anchor(item.id.clone(), item.anchor)]
}
//#endregion 🔖️Inverse
