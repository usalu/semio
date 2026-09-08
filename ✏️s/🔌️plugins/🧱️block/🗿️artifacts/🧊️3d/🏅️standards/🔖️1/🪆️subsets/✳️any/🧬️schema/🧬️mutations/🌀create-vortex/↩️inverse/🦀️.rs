//! ↩️ Inverse for `CreateVortex`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::CreateVortex, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_vortex::delete_vortex(payload.vortex.id.clone())]
}
//#endregion 🔖️Inverse
