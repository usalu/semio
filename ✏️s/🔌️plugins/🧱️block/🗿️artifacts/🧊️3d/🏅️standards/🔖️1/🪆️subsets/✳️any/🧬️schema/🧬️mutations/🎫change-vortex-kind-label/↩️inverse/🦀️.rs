//! ↩️ Inverse for `ChangeVortexKindLabel`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeVortexKindLabel, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_kind_label::change_vortex_kind_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
