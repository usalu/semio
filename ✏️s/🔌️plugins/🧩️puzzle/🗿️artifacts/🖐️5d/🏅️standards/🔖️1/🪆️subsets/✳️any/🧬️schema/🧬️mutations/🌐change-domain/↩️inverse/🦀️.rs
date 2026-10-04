//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle5dMutation;
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse(_payload: &super::ChangeDomain, base: &Puzzle5dSnapshot) -> Result<Vec<Puzzle5dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_domain::change_domain(base.domain.clone())]

    })())
}
//#endregion 🔖️Inverse
