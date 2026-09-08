//! ↩️ Inverse for `DeleteVortexKind`.

use crate::Block3dSnapshot;
use crate::mutations::Block3dMutation;

//#region 🔖️Inverse
pub fn inverse(payload: &super::DeleteVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::create_vortex_kind::create_vortex_kind(existing.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
