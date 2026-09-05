//! ↩️ Inverse for `ConnectVortices` — always a `disconnect-vortices` of the id it created.
use crate::artifacts::puzzle3d::mutations::Puzzle3dMutation;
use crate::artifacts::puzzle3d::Puzzle3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ConnectVortices, _base: &Puzzle3dSnapshot) -> Vec<Puzzle3dMutation> {
    vec![crate::artifacts::puzzle3d::mutations::disconnect_vortices::mutation::disconnect_vortices(payload.id.clone())]
}
//#endregion 🔖️Inverse
