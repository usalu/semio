//! ↩️ Inverse for `ChangePart2dHidden` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangePart2dHidden, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::mutations::change_part_2d_hidden::change_part_2d_hidden(item.id.clone(), item.part_2d.hidden)]
}
//#endregion 🔖️Inverse
