//! ↩️ Inverse for `ChangeDescription` — restores the BASE description.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::ChangeDescription, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_description::ChangeDescription>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_description::change_description(base.meta.description.clone())]

    })())
}
//#endregion 🔖️Inverse
