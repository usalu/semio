//! ↩️ Inverse for `CreateVortex`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateVortex, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_vortex::delete_vortex(payload.vortex.id.clone())]
}
//#endregion 🔖️Inverse
