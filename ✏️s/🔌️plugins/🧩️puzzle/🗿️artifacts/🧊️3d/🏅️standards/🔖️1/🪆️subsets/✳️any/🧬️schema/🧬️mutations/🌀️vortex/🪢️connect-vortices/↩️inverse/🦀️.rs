//! ↩️ Inverse for `ConnectVortices` — always a `disconnect-vortices` of the id it created.
use super::mutation::ConnectVortices;
use crate::standards::v1::subsets::any::schema::mutations::DisconnectVortices;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ConnectVortices> + From<DisconnectVortices>>(payload: &super::mutation::ConnectVortices, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::disconnect_vortices(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse
