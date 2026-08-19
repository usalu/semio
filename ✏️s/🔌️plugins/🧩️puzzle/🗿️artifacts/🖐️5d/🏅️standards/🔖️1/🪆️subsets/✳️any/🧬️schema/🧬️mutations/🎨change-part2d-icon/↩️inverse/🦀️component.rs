//! ↩️ Inverse for `ChangePart2dIcon` — restores the BASE field value. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangePart2dIcon, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(item) = base.parts.iter().find(|entry| entry.id == payload.id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::change_part_2d_icon::mutation::change_part_2d_icon(item.id.clone(), item.part_2d.icon_kind.clone())]
}
//#endregion 🔖️Inverse
