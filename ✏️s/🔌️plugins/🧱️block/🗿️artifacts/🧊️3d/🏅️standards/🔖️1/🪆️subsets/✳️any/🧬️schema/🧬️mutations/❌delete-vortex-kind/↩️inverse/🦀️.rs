//! ↩️ Inverse for `DeleteVortexKind`.

use crate::artifacts::block3d::Block3dSnapshot;
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::DeleteVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_vortex_kind::create_vortex_kind(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
