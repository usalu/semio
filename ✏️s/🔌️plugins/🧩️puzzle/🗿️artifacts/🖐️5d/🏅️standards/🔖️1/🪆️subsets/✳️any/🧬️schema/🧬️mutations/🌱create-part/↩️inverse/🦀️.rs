//! ↩️ Inverse for `CreatePart` — always a `delete-part` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreatePart, _base: &Puzzle5dSnapshot) -> Vec<Puzzle5dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_part::delete_part(payload.part.id.clone())]
}
//#endregion 🔖️Inverse
