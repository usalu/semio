//! ↩️ Inverse for `ChangeVortexKindDefaultCableKind`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeVortexKindDefaultCableKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_kind_default_cable_kind::change_vortex_kind_default_cable_kind(payload.id.clone(), existing.default_cable_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
