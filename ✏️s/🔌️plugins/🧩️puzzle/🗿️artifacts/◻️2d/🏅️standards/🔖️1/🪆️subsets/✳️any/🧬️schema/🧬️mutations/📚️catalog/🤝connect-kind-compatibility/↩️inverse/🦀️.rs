//! pair it created.
//! ↩️ Inverse for `ConnectKindCompatibility` — always a `disconnect-kind-compatibility` of the
use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility;
use crate::Puzzle2dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ConnectKindCompatibility> + From<DisconnectKindCompatibility>>(payload: &super::ConnectKindCompatibility, _base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::disconnect_kind_compatibility(payload.source.clone(), payload.target.clone())]

    })())
}
//#endregion 🔖️Inverse
