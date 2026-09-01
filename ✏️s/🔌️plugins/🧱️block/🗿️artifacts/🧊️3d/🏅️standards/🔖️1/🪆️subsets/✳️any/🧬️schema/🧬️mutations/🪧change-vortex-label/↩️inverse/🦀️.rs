//! ↩️ Inverse for `ChangeVortexLabel`.

use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexTemplate};
use crate::artifacts::block3d::mutations::Block3dMutation;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::ChangeVortexLabel, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_label::change_vortex_label(payload.id.clone(), existing.label.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
