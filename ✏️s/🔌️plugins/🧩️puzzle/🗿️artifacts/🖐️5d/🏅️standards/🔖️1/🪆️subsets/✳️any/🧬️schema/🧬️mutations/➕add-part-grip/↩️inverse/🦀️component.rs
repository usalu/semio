//! ↩️ Inverse for `AddPartGrip` — always a `remove-part-grip` of the grip it added.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddPartGrip, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::remove_part_grip::mutation::remove_part_grip(payload.part_id.clone(), payload.grip.id.clone())]
}
//#endregion 🔖️Inverse
