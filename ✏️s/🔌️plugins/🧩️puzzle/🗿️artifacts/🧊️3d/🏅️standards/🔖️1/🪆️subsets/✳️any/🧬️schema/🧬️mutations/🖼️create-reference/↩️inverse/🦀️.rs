//! ↩️ Inverse for `CreateReference` — always a `delete-reference` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateReference, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::delete_reference(payload.reference.id.clone())]
}
//#endregion 🔖️Inverse
