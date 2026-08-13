//! ↩️ Inverse for `RenameVortexKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::RenameVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match crate::artifacts::block3d::vortex_kinds_of(base).iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::rename_vortex_kind::mutation::rename_vortex_kind(payload.id.clone(), existing.name.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
