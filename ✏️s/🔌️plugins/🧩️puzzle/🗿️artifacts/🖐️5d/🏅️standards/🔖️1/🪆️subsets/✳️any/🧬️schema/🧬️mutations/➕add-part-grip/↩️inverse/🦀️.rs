//! ↩️ Inverse for `AddPartGrip` — always a `remove-part-grip` of the grip it added.
use crate::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::AddPartGrip, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::mutations::remove_part_grip::remove_part_grip(payload.part_id.clone(), payload.grip.id.clone())]
}
//#endregion 🔖️Inverse
