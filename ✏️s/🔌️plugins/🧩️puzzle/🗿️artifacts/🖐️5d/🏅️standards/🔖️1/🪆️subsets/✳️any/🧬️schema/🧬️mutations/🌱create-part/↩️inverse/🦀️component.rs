//! ↩️ Inverse for `CreatePart` — always a `delete-part` of the id it created.
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreatePart, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::artifacts::puzzle5d::mutations::delete_part::mutation::delete_part(payload.part.id.clone())]
}
//#endregion 🔖️Inverse
