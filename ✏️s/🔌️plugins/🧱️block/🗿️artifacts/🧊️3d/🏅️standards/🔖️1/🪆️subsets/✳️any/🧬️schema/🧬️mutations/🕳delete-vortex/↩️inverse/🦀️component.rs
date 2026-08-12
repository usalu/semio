//! ↩️ Inverse for `DeleteVortex` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::DeleteVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::create_vortex::mutation::create_vortex(existing.clone())], None => Vec::new() }
}
//#endregion 🔖️Inverse
