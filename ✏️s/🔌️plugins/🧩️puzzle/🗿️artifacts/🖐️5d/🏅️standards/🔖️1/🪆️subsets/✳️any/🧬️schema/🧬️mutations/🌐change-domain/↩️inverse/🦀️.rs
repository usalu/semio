//! ↩️ Inverse for `ChangeDomain` — restores the BASE domain.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(_payload: &super::ChangeDomain, base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_domain::ChangeDomain>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::change_domain::change_domain(base.domain.clone())]

    })())
}
//#endregion 🔖️Inverse
