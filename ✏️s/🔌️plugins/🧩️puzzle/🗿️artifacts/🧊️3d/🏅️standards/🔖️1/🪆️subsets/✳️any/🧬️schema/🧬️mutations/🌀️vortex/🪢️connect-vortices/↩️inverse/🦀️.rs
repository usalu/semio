//! ↩️ Inverse for `ConnectVortices` — always a `disconnect-vortices` of the id it created.
use super::mutation::ConnectVortices;
use crate::disconnect_vortices::mutation::DisconnectVortices;
use semio_s_artifact_puzzle_3d_model::snapshot::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<ConnectVortices> + From<DisconnectVortices>>(payload: &super::mutation::ConnectVortices, _base: &Puzzle3dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
    vec![crate::disconnect_vortices::mutation::disconnect_vortices(payload.id.clone())]

    })())
}
//#endregion 🔖️Inverse
