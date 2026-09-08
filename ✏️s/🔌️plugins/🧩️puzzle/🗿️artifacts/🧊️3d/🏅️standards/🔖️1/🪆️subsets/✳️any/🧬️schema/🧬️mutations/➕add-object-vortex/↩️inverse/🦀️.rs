//! ↩️ Inverse for `AddObjectVortex` — always a `remove-object-vortex` of the vortex it added.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddObjectVortex, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::remove_object_vortex(payload.object_id.clone(), payload.vortex.id.clone())]
}
//#endregion 🔖️Inverse
