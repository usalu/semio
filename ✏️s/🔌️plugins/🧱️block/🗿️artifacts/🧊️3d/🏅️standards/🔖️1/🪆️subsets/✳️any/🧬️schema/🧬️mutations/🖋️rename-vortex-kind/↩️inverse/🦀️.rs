//! ↩️ Inverse for `RenameVortexKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::RenameVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::rename_vortex_kind::rename_vortex_kind(payload.id.clone(), existing.name.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
