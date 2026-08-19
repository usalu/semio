//! ↩️ Inverse for `CreateVortex` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::CreateVortex, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_vortex::mutation::delete_vortex(payload.vortex.id.clone())]
}
//#endregion 🔖️Inverse
