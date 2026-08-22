//! ↩️ Inverse for `AddObjectVortex` — always a `remove-object-vortex` of the vortex it added.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::AddObjectVortex, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::remove_object_vortex::mutation::remove_object_vortex(payload.object_id.clone(), payload.vortex.id.clone())]
}
//#endregion 🔖️Inverse
