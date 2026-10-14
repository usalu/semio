//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use super::mutation::ChangeDomain;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ChangeDomain>>(_payload: &super::mutation::ChangeDomain, base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_domain::mutation::change_domain(base.domain.clone())]

    })())
}
//#endregion 🔖️Inverse
