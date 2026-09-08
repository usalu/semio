//! ↩️ Inverse for `ChangeVortexVortexKind`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::ChangeVortexVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_vortex_kind::change_vortex_vortex_kind(payload.id.clone(), existing.vortex_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
