//! ↩️ Inverse for `ChangeVortexVortexKind` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub async fn inverse(payload: &super::mutation::ChangeVortexVortexKind, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) {
        Some(existing) => vec![super::super::change_vortex_vortex_kind::mutation::change_vortex_vortex_kind(payload.id.clone(), existing.vortex_kind.clone())],
        None => Vec::new(),
    }
}
//#endregion 🔖️Inverse
