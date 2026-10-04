//! ↩️ Inverse for `ChangeDescription` — restores the BASE description.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeDescription, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_description::change_description(base.meta.description.clone())]

    })())
}
//#endregion 🔖️Inverse
