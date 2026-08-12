//! ↩️ Inverse for `RotatePart3d` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RotatePart3d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::rotate_part_3d::mutation::rotate_part_3d(item.id.clone(), item.part_3d.orientation)]
}
//#endregion 🔖️Inverse
