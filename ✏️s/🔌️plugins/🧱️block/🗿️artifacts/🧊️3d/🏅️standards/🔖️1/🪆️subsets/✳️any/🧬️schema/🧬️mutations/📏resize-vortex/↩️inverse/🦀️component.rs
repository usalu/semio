//! ↩️ Inverse for `ResizeVortex` — reconstructed from `base` (pre-state) only.
use crate::artifacts::block3d::mutations::Block3dMutation;
use crate::artifacts::block3d::Block3dSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &super::mutation::ResizeVortex, base: &Block3dSnapshot) -> Vec<Block3dMutation> {
    match base.vortices.iter().find(|item| item.id == payload.id) { Some(existing) => vec![super::super::resize_vortex::mutation::resize_vortex(payload.id.clone(), existing.radius)], None => Vec::new() }
}
//#endregion 🔖️Inverse
