//! ↩️ Inverse for `CreateVortexKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::CreateVortexKind, _base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    vec![super::super::delete_vortex_kind::delete_vortex_kind(payload.vortex_kind.id.clone())]
}
//#endregion 🔖️Inverse
