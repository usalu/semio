//! ↩️ Inverse for `AddObjectVortex` — always a `remove-object-vortex` of the vortex it added.
use super::mutation::AddObjectVortex;
use crate::standards::v1::subsets::any::schema::mutations::RemoveObjectVortex;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<AddObjectVortex> + From<RemoveObjectVortex>>(payload: &super::mutation::AddObjectVortex, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::remove_object_vortex::mutation::remove_object_vortex(payload.object_id.clone(), payload.vortex.id.clone())]

    })())
}
//#endregion 🔖️Inverse
