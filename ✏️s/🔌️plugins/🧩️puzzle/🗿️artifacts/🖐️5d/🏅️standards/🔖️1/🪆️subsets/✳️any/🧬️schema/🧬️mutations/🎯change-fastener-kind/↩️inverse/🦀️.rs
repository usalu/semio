//! ↩️ Inverse for `ChangeFastenerKind` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeFastenerKind, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.fasteners.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_fastener_kind::change_fastener_kind(item.id.clone(), item.fastener_kind.clone())]
}
//#endregion 🔖️Inverse
