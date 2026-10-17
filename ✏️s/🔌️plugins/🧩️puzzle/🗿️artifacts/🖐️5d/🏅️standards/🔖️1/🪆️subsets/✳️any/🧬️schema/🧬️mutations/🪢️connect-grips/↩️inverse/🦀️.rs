//! ↩️ Inverse for `ConnectGrips` — always a `disconnect-grips` of the id it created.
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ConnectGrips, _base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::DisconnectGrips>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_grips::disconnect_grips(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse
