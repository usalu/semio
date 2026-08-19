//! ↩️ Inverse for `MovePart2d` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::MovePart2d, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::move_part_2d::mutation::move_part_2d(item.id.clone(), item.part_2d.x, item.part_2d.y)]
}
//#endregion 🔖️Inverse
