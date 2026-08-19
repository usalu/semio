//! ↩️ Inverse for `CreateObject` — always a `delete-object` of the id it created.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateObject, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::delete_object::mutation::delete_object(payload.object.id.clone())]
}
//#endregion 🔖️Inverse
