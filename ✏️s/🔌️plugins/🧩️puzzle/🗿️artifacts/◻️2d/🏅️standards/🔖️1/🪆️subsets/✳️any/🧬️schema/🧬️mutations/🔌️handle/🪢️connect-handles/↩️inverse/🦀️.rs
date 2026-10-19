//! ↩️ Inverse for `ConnectHandles` — always a `disconnect-handles` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::DisconnectHandles;
use crate::Puzzle2dSnapshot;
use crate::standards::v1::subsets::any::schema::mutations::connect_handles::component::ConnectHandles;

//#region 🔖️Inverse
pub fn inverse<Op: From<ConnectHandles> + From<DisconnectHandles>>(payload: &super::ConnectHandles, _base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_handles::disconnect_handles(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse
