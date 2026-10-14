//! ↩️ Inverse for `CreateReference` — always a `delete-reference` of the id it created.
use super::mutation::CreateReference;
use crate::standards::v1::subsets::any::schema::mutations::DeleteReference;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<CreateReference> + From<DeleteReference>>(payload: &super::mutation::CreateReference, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_reference::mutation::delete_reference(payload.reference.id.clone())]

    })())
}
//#endregion 🔖️Inverse
