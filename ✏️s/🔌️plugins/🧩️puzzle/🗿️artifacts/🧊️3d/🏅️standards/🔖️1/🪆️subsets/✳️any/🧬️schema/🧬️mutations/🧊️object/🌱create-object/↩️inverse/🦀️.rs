//! ↩️ Inverse for `CreateObject` — always a `delete-object` of the id it created.
use super::mutation::CreateObject;
use crate::standards::v1::subsets::any::schema::mutations::DeleteObject;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateObject> + From<DeleteObject>>(payload: &super::mutation::CreateObject, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_object::mutation::delete_object(payload.object.id.clone())]

    })())
}
//#endregion 🔖️Inverse
