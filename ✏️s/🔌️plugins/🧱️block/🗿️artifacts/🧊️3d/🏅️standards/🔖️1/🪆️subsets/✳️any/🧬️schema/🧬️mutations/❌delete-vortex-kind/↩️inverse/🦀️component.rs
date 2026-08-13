//! ↩️ Inverse for `DeleteVortexKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::create_vortex_kind::mutation::create_vortex_kind(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
