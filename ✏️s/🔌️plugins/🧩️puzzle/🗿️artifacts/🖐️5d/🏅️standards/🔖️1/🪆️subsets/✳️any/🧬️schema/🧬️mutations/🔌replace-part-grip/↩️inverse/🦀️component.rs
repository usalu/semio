//! ↩️ Inverse for `ReplacePartGrip` — restores the BASE grip payload. Missing target ⇒ `Vec::new()`.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ReplacePartGrip, base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    let Some(part) = base.parts.iter().find(|entry| entry.id == payload.part_id) else {
        return Vec::new();
    };
    let Some(grip) = part.grips.iter().find(|grip| grip.id == payload.grip_id) else {
        return Vec::new();
    };
    vec![crate::artifacts::puzzle5d::mutations::replace_part_grip::mutation::replace_part_grip(payload.part_id.clone(), payload.grip_id.clone(), grip.clone())]
}
//#endregion 🔖️Inverse
