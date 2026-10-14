//! ↩️ Inverse for `CreateObject` — always a `delete-object` of the id it created.
use super::mutation::CreateObject;
use crate::delete_object::mutation::DeleteObject;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateObject> + From<DeleteObject>>(payload: &super::mutation::CreateObject, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::delete_object::mutation::delete_object(payload.object.id.clone())]

    })())
}
//#endregion 🔖️Inverse
