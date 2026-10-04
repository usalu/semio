//! ↩️ Inverse for `ConnectVortices` — always a `disconnect-vortices` of the id it created.
use crate::standards::v1::subsets::any::schema::mutations::Puzzle3dMutation;
use crate::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectVortices, _base: &Puzzle3dSnapshot) -> Result<Vec<Puzzle3dMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::standards::v1::subsets::any::schema::mutations::disconnect_vortices::mutation::disconnect_vortices(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse
