//! ↩️ Inverse for `CreatePart` — always a `delete-part` of the id it created.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::CreatePart, _base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::delete_part::DeletePart>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::delete_part::delete_part(payload.part.id.clone())]

    })())
}
//#endregion 🔖️Inverse
