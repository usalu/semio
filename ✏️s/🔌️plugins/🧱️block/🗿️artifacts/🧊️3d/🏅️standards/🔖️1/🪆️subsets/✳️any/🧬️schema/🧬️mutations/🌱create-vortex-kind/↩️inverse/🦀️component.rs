//! ↩️ Inverse for `CreateVortexKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::CreateVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_vortex_kind::mutation::delete_vortex_kind(payload.vortex_kind.id.clone())]
}
//#endregion 🔖️Inverse
