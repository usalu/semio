//! 👫️ pair it created.
//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
use crate::Puzzle5dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op>(payload: &super::ConnectKindCompatibility, _base: &Puzzle5dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility>,
{
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]

    })())
}
//#endregion 🔖️Inverse
